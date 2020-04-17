export const gigasecond = startDate => {
  const MILLISECS_PER_GIGASEC = 1e12;
  const gigasecLater = new Date(startDate.getTime() + MILLISECS_PER_GIGASEC);

  return gigasecLater;
};
