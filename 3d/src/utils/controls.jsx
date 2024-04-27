export function getControlValue({value = 0, step = 0.01, min = 0.1, max = 1}) {
  return {value, min, max, step}
}
