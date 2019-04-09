export const changeShowReview = ({ state, commit }, isShow) => {
  if (!!isShow) {
    commit("changeShowReview", true);
  } else {
    commit("changeShowReview", false);
  }
};

export const toggleLockingReview = ({ state, commit }) => {
  commit("toggleLockingReview");
};

export const changeScreenWidth = ({state, commit }, screenWidth) => {
  commit("changeSreenSize", screenWidth);
};

export const fetchBackgrounds = ({commit}) => {
  console.log(this);
  // context.$axios().$get("http://127.0.0.1:3001/api/backgrounds").then(response => {
  //   console.log(response.data);
  //   commit("setBackgrounds", response.data);
  // });
};

export const fetchReviews = ({commit}) => {
  // this.$axios.$get("http://127.0.0.1:3001/api/reviews").then(response => {
  //   console.log(response.data);
  //   commit("setReviews", response.data);
  // }).catch(e => error(e));
};
