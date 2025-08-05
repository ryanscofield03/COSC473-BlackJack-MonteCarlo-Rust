const formatEstimatedValue = (number: number): string => {
  if (number < 0) {
    return '-$' + (-1 * number).toFixed(2)
  } else {
    return '$' + number.toFixed(2)
  }
}

const formatProbabilityPercentage = (number: number): string => {
  return (number * 100).toFixed(2) + '%'
}

export { formatEstimatedValue, formatProbabilityPercentage }