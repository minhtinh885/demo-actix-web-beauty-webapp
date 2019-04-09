export const changeShowReview = (state, isShow) => {
  if (!state.isLockingReview) {
    state.isShowReview = isShow;
  }
};

export const toggleLockingReview = (state) => {
  state.isLockingReview = !state.isLockingReview
};

export const changeSreenSize = (state, width) => {
  if (width !== 0) {
    state.screenWidth = width;
  }
};

export const setBackgrounds = (state, backgrounds) => {
  state.backgrounds = backgrounds;
};

export const setReviews = (state, reviews) => {
  state.reviews = reviews;
};
