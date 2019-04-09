import Vuex from 'vuex';
import Cookie from 'js-cookie';

const store = () => {
  return new Vuex.Store({
    state: {
      screenWidth: 0,
      isShowReview: false,
      isLockingReview: false,
      images: [],
      reviews: [],
      backgrounds: [],
      fullReviews: [],
      token: null,
    },
    mutations: {
      setToken(state, token) {
        state.token = token;
      },
      clearToken(state) {
        state.token = null;
      },
      changeShowReview(state, isShow) {
        if (!state.isLockingReview) {
          state.isShowReview = isShow;
        }
      },
      toggleLockingReview(state){
        state.isLockingReview = !state.isLockingReview
      },
      changeSreenSize(state, width){
        if (width !== 0) {
          state.screenWidth = width;
        }
      },
      setImages(state, images){
        state.images = images;
      },
      setReviews(state, reviews){
        state.reviews = reviews;
      },
      // Backgrounds
      setBackgrounds(state, backgrounds){
        state.backgrounds = backgrounds;
      },
      removeBackground(state, _id) {
        let index = state.backgrounds.findIndex(x => x.id === _id);
        state.backgrounds.splice(index, 1);
      },
      changeBackgroundStatus(state, payload) {
        let index = state.backgrounds.findIndex(x => x.id === payload.id);
        state.backgrounds[index].status = payload.status;
      },
      addNewBackground(state, payload) {
        state.backgrounds.push(payload.background);
      },
      updateNewBackground(state, payload) {
        let index = state.backgrounds.findIndex(x => x.id === payload.background.id);
        state.backgrounds.splice(index, 1, payload.background);
      },

      // Review
      setFullReviews(state, reviews){
        state.fullReviews = reviews;
      },
      removeReview(state, _id) {
        let index = state.fullReviews.findIndex(x => x.id === _id);
        state.fullReviews.splice(index, 1);
      },
      changeReviewStatus(state, payload) {
        let index = state.fullReviews.findIndex(x => x.id === payload.id);
        state.fullReviews[index].status = payload.status;
      },
      addNewReview(state, payload) {
        state.fullReviews.push(payload.review);
      },
      updateNewReview(state, payload) {
        let index = state.fullReviews.findIndex(x => x.id === payload.review.id);
        state.fullReviews.splice(index, 1, payload.review);
      }
    },
    actions: {
      authenticateUser({commit}, authData) {
        let data = JSON.stringify(authData);
        let url = process.env.baseUrl + '/api/auth';
        return new Promise((resolve, reject) => {
          this.$axios.$post(url,
            data,
            {
              headers: {
                "Content-Type": "application/json",
              }}
          ).then(response => {
            Cookie.set("auth", response);
            Cookie.set("expirationDate", new Date().getTime() + (24 * 60 * 60 * 1000) - 15000);
            commit("setToken", response);
            resolve();
          }, e => reject(e));
        });
      },
      initAuth(context, req) {
        /*let token = Cookie.get("auth");
        console.log(token);
        if (token !== undefined) {
          let expirationDate = Cookie.get("expirationDate");
          if (expirationDate) {
            if (expirationDate <= (new Date().getTime())) {
              commit("clearToken");
            } else {
              commit("setToken", token);

            }
          }
        }*/

        let token;
        let expirationDate;
        if (req) {
          if (!req.headers.cookie) {
            return;
          }
          const jwtCookie = req.headers.cookie
            .split(";")
            .find(c => c.trim().startsWith("auth="));
          if (!jwtCookie) {
            return;
          }
          token = jwtCookie.split("=")[1];
          expirationDate = req.headers.cookie
            .split(";")
            .find(c => c.trim().startsWith("expirationDate="))
            .split("=")[1];
        } else if (process.client) {
          token = Cookie.get("auth");
          if (token !== undefined) {
            let expirationDate = Cookie.get("expirationDate");
            if (expirationDate) {
              if (expirationDate <= (new Date().getTime())) {
                context.commit("clearToken");
              } else {
                context.commit("setToken", token);

              }
            }
          }
        }
        if (new Date().getTime() > +expirationDate || !token) {
          console.log("No token or invalid token");
          context.dispatch("logout");
          return;
        }
        context.commit("setToken", token);
      },

      logout({commit}) {
        let url = process.env.baseUrl + '/api/auth';
        this.$axios.$delete(url).then(response => {
          commit("clearToken");
          Cookie.remove("auth");
          Cookie.remove("expirationDate");
        }).catch(e => console.log(e));
      },
      changeShowReview({ state, commit }, isShow){
        if (!!isShow) {
          commit("changeShowReview", true);
        } else {
          commit("changeShowReview", false);
        }
      },
      toggleLockingReview({ state, commit }){
        commit("toggleLockingReview");
      },
      changeScreenWidth({state, commit }, screenWidth){
        commit("changeSreenSize", screenWidth);
      },
      fetchImages({commit}){
        let url = process.env.baseUrl + '/api/images';
        return this.$axios.$get(url).then(response => {
          commit("setImages", response);
        }).catch(e => console.log(e));
      },
      // Background
      fetchBackgrounds({commit, state}){
        let url = process.env.baseUrl + '/api/background';
        return this.$axios.$get(url, {
          headers: {
            'Authorization': state.token,
          }
        }).then(response => {
          commit("setBackgrounds", response);
        }).catch(e => console.log(e));
      },
      removeBackground({commit, state}, id) {
        let url = process.env.baseUrl + '/api/background/' + id;
        return new Promise((resolve, reject) => {
          this.$axios.$delete(url, {
            headers: {
              'Authorization': state.token,
            }
          }).then(() => {
            commit("removeBackground", id);
            resolve();
          }, (error) => reject(error));
        });
      },
      changeBackgroundStatus({commit, state}, payload) {
        let data = {
          status: payload.status,
        };
        let url = process.env.baseUrl + '/api/background/' + payload.id;
        return this.$axios.$patch(url,
          JSON.stringify(data), {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
          commit("changeBackgroundStatus", payload);
        }).catch(e => error(e));
      },

      SubmitBackgroundForm({commit, state}, payload) {
        let formData = new FormData();
        let createBackground = {};
        Object.keys(payload.background).forEach(function(item) {
          if (item !== 'id') {
            createBackground[item] = payload.background[item];
          }
        });

        if (createBackground.status === '1') {
          createBackground.status = 1;
        } else if (createBackground.status === '0') {
          createBackground.status = 0;
        }

        if (payload.file) {
          createBackground.image_url = payload.file.name;
        }
        let data = JSON.stringify(createBackground);
        formData.append('file', payload.file);
        let url_upload = process.env.baseUrl + '/api/upload';
        let url_background = process.env.baseUrl + '/api/background';
        if (payload.background.id === -1) {

          this.$axios.$post(url_upload, formData, {headers: {'Content-Type': 'multipart/form-data', 'Authorization': state.token}})
            .then(response => {
            }).catch(e => console.log(e));

          this.$axios.$post(url_background,
            data,
            {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
            commit("addNewBackground", {background: response});
          }).catch(e => console.log(e));

        } else {
          if (payload.file !== null && payload.file.name !== payload.background.image_url) {
            this.$axios.$post(url_upload, formData, {headers: {'Content-Type': 'multipart/form-data', 'Authorization': state.token}})
              .then(response => {
              }).catch(e => console.log(e));
          }

          this.$axios.$post(url_background + '/' + payload.background.id,
            data,
            {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
            commit("updateNewBackground", {background: response});
          }).catch(e => console.log(e));
        }
      },

      // Review
      fetchReviews({commit}){
        let url = process.env.baseUrl + '/api/reviews';
        return this.$axios.$get(url).then(response => {
          commit("setReviews", response);
        }).catch(e => console.log(e));
      },
      fetchFullReviews({commit, state}){
        let url = process.env.baseUrl + '/api/full-reviews';
        return this.$axios.$get(url, {
          headers: {
            'Authorization': state.token,
          }
        }).then(response => {
          commit("setFullReviews", response);
        }).catch(e => console.log(e));
      },
      removeReview({commit, state}, id) {
        let url = process.env.baseUrl + '/api/full-reviews/' + id;
        return new Promise((resolve, reject) => {
          this.$axios.$delete(url, {
            headers: {
              'Authorization': state.token,
            }
          }).then(() => {
            commit("removeReview", id);
            resolve();
          }, (error) => reject(error));
        });
      },
      changeReviewStatus({commit, state}, payload) {
        let data = {
          status: payload.status,
        };
        let url = process.env.baseUrl + '/api/full-reviews/' + payload.id;

        return this.$axios.$patch(url,
          JSON.stringify(data), {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
          commit("changeReviewStatus", payload);
        }).catch(e => error(e));
      },
      SubmitReviewForm({commit, state}, payload) {
        let formData = new FormData();
        let createReview = {};
        Object.keys(payload.review).forEach(function(item) {
          if (item !== 'id') {
            createReview[item] = payload.review[item];
          }
        });

        if (createReview.gender === '1') {
          createReview.gender = 1;
        } else if (createReview.gender === '0') {
          createReview.gender = 0;
        }

        if (payload.file) {
          createReview.image_url = payload.file.name;
        }
        let data = JSON.stringify(createReview);
        formData.append('file', payload.file);

        let url_upload = process.env.baseUrl + '/api/upload';
        let url_review = process.env.baseUrl + '/api/full-reviews';

        if (payload.review.id === -1) {

          this.$axios.$post(url_upload, formData, {headers: {'Content-Type': 'multipart/form-data', 'Authorization': state.token}})
            .then(response => {
            }).catch(e => console.log(e));

          this.$axios.$post(url_review,
            data,
            {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
              commit("addNewReview", {review: response});
          }).catch(e => console.log(e));

        } else {
          if (payload.file !== null && payload.file.name !== payload.review.image_url) {
            this.$axios.$post(url_upload, formData, {
              headers: {
                'Authorization': state.token,
                'Content-Type': 'multipart/form-data'
              }
            })
              .then(response => {
              }).catch(e => console.log(e));
          }

          this.$axios.$post(url_review + '/' + payload.review.id,
            data,
            {headers: {'Content-Type': 'application/json', 'Authorization': state.token}}).then(response => {
            commit("updateNewReview", {review: response});
          }).catch(e => console.log(e));
        }
      }
    },
    getters: {
      isShowingReview(state) {
        return state.isShowReview;
      },

      isLockingReview(state) {
        return state.isLockingReview;
      },

      getScreenWidth(state) {
        return state.screenWidth;
      },

      getImages(state) {
        return state.images;
      },
      getBackgrounds(state) {
        return state.backgrounds;
      },

      getReviews(state) {
        return state.reviews;
      },
      getFullReviews(state) {
        return state.fullReviews;
      },
      isAuthenticated(state) {
        return state.token != null;
      }
    },
  });
};

export default store;
