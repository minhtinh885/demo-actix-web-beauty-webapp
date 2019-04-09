const debounce = (fn, delay) => {
  let timeoutId = null;
  return function () {
    clearTimeout(timeoutId);
    let args = arguments;
    let that = this;
    timeoutId = setTimeout(function () {
      fn.apply(that, args);
    }, delay);
  }
};

export default debounce;
