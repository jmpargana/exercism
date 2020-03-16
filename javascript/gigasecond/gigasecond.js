export const gigasecond = (startDate) => {
  let endDate = new Date(startDate);
  endDate.setTime( startDate.getTime() + Math.pow(10, 12) );

  return endDate
};
