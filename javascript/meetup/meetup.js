// meetup.js

export const meetupDay = (year, month, weekday, descriptor) => {
  const filterForDescriptor = filtersForDescriptors[descriptor];
  const day = daysInYearMonth(year, month)
    .filter(day => day.getDay() == weekdays[weekday])
    .filter(filterForDescriptor)
    .last();

  if (day === undefined) {
    throw "Invalid date";
  }

  return day;
};

const daysInYearMonth = (years, month) => {
  const days = new Array();
  const start = new Date(years, month);

  var daysCounted = 0;
  do {
    days.push(start.addDays(daysCounted));
    daysCounted++;
  } while (days.last().getMonth() == month);

  return days;
};

const filtersForDescriptors = {
  teenth: date => 12 < date.getUTCDate() && date.getUTCDate() < 20,
  "1st": date => date.getWeek() == 1,
  "2nd": date => date.getWeek() == 2,
  "3rd": date => date.getWeek() == 3,
  "4th": date => date.getWeek() == 4,
  "5th": date => date.getWeek() == 5,
  last: date => date.addDays(7).getMonth() != date.getMonth()
};

if (!Array.prototype.last) {
  Array.prototype.last = function() {
    return this[this.length - 1];
  };
}

if (!Date.prototype.addDays) {
  Date.prototype.addDays = function(days) {
    var date = new Date(this.valueOf());
    date.setDate(date.getDate() + days);
    return date;
  };
}

if (!Date.prototype.getWeek) {
  Date.prototype.getWeek = function() {
    return Math.floor((this.getUTCDate() - 1) / 7) + 1;
  };
}

const weekdays = {
  Sunday: 0,
  Monday: 1,
  Tuesday: 2,
  Wednesday: 3,
  Thursday: 4,
  Friday: 5,
  Saturday: 6
};
