use std::collections::{HashMap, HashSet};

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

type Computation<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;
type Callback<'a, T> = Box<dyn FnMut(T) -> () + 'a>;

struct ComputeCell<'a, T> {
    value: T,
    dependencies: HashSet<CellID>,
    input_ids: Vec<CellID>,
    computation: Computation<'a, T>,
    callbacks: HashMap<CallbackID, Callback<'a, T>>,
    next_cb_id: CallbackID,
}

impl<'a, T> ComputeCell<'a, T> {
    fn new(value: T, input_ids: &[CellID], computation: Computation<'a, T>) -> Self {
        Self {
            value,
            dependencies: input_ids.iter().cloned().collect::<HashSet<_>>(),
            input_ids: input_ids.to_vec(),
            computation,
            callbacks: HashMap::new(),
            next_cb_id: CallbackID(0),
        }
    }

    fn unique_cb_id(&mut self) -> CallbackID {
        let inner = self.next_cb_id.0;

        self.next_cb_id = CallbackID(inner + 1);
        CallbackID(inner)
    }

    fn add_callback(&mut self, new_callback: Callback<'a, T>) -> CallbackID {
        let id = self.unique_cb_id();
        self.callbacks.insert(id, new_callback);
        id
    }

    fn remove_callback(&mut self, id: CallbackID) -> Result<(), RemoveCallbackError> {
        self.callbacks
            .remove(&id)
            .map(|_| ())
            .ok_or(RemoveCallbackError::NonexistentCallback)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn unwrap(self) -> usize {
        match self {
            CellID::Input(inner) => inner.0,
            CellID::Compute(inner) => inner.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    input_cells: Vec<T>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + 'a> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.input_cells.push(initial);
        InputCellID(self.input_cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let dependent_vals = &dependencies
            .iter()
            .map(|&id| self.value(id).ok_or(id))
            .collect::<Result<Vec<_>, CellID>>()?;

        let initial_value = compute_func(dependent_vals);
        let compute_cell = ComputeCell::new(initial_value, dependencies, Box::new(compute_func));
        self.compute_cells.push(compute_cell);
        Ok(ComputeCellID(self.compute_cells.len() - 1))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(id) => self.input_cells.get(id.0).copied(),
            CellID::Compute(id) => self.compute_cells.get(id.0).map(|cell| cell.value),
        }
    }

    fn update_compute(&mut self, id: InputCellID) -> Option<()> {
        // create a tiered list of dependencies where higher values
        // are farther than the initial id
        let mut current_dependencies = [CellID::Input(id)].iter().cloned().collect::<HashSet<_>>();
        let mut dependency_lvls = vec![current_dependencies];
        loop {
            current_dependencies = dependency_lvls[dependency_lvls.len() - 1].clone();
            let next_dependencies = self
                .compute_cells
                .iter()
                .enumerate()
                .filter_map(|(id, candidate)| {
                    if candidate.dependencies.is_disjoint(&current_dependencies) {
                        None
                    } else {
                        Some(CellID::Compute(ComputeCellID(id)))
                    }
                })
                .collect::<HashSet<_>>();

            if next_dependencies.is_empty() {
                break;
            } else {
                dependency_lvls.push(next_dependencies);
            }
        }

        // filter out repeats so they only show up in farthest
        // dependency level
        let update_order = dependency_lvls
            .into_iter()
            .rev()
            .scan(
                HashSet::new(),
                |already_used: &mut HashSet<CellID>, candidates: HashSet<CellID>| {
                    let this_level = candidates
                        .iter()
                        .cloned()
                        .filter(|candidate| !already_used.contains(candidate))
                        .collect::<HashSet<_>>();
                    already_used.extend(this_level.clone());
                    Some(this_level)
                },
            )
            .collect::<Vec<_>>();

        // update all dependencies in selected order
        update_order
            .iter()
            .rev()
            .flatten()
            .skip(1)
            .map(|id| {
                let cell = self.compute_cells.get(id.unwrap())?;
                let computation = &cell.computation;
                let inputs = cell
                    .input_ids
                    .iter()
                    .map(|&id| self.value(id))
                    .collect::<Option<Vec<_>>>()?;
                let value = computation(&inputs);
                if cell.value != value {
                    let cell = self.compute_cells.get_mut(id.unwrap())?;
                    cell.value = value;

                    cell.callbacks
                        .values_mut()
                        .for_each(|callback| callback(value));
                }
                Some(())
            })
            .collect::<Option<Vec<_>>>()?;
        Some(())
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let input_success = self
            .input_cells
            .get_mut(id.0)
            .map(|cell_value| *cell_value = new_value)
            .is_some();

        let update_success = self.update_compute(id).is_some();

        input_success && update_success
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.compute_cells
            .get_mut(id.0)
            .map(|cell| cell.add_callback(Box::new(callback)))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellID,
        callback_id: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get_mut(cell_id.0)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| cell.remove_callback(callback_id))
    }
}
