<template>
  <div>
    <nav-header/>
    <section class="hero">
      <div class="bg-carousel">
        <no-ssr><bg-carousel v-if="images" :backgrounds="images"/></no-ssr>
      </div>
      <div class="side-bar">
        <navigation/>
        <div class="feed-back hide-sm">
          <button class="button-aqua" @click="showReview(!isReview)">
            <img class="icon icon-large" src="icons/cam-nhan.svg" alt="Cảm nhận khách hàng">
            <!--<object class="icon icon-large" data="icons/cam-nhan.svg" type="image/svg+xml"></object>-->
            <span>Cảm nhận khách hàng</span>
          </button>
        </div>
      </div>

      <nuxt/>
    </section>

    <no-ssr><reviews v-if="reviews" :reviews="reviews"/></no-ssr>
    <nav-bottom/>

  </div>
</template>
<script>
  export default {
    head() {
      return {
        link: [
          { rel: 'stylesheet', href: '/css/main.css' },
          { rel: 'stylesheet', href: '/css/swiper.min.css' }
        ]
      }
    },
    computed: {
      isReview() {
        return this.$store.getters.isShowingReview;
      },
      images() {
        return this.$store.getters.getImages;
      },
      reviews() {
        return this.$store.getters.getReviews;
      }
    },
    methods: {
      showReview(isShow) {
        this.$store.dispatch("changeShowReview", isShow);
      }
    }
  }
</script>
<style>

</style>
