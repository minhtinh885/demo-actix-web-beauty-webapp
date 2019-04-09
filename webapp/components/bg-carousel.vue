<template>
  <div>
    <!--<no-ssr>-->
      <!--<carousel :autoplay="true" :perPage="1" :paginationPadding="2" :paginationSize="screenWidth < 768 ? 3 : 6" :autoplayTimeout="10000" :paginationEnabled="(screenWidth > 0 && screenWidth < 768) || !isShowingReview">
        <slide v-for="(item, key) of items" :key="key">
          <div :style="{
          backgroundImage: 'url(' + item.img + ')',
          backgroundSize: 'cover',
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'center',
          height: '100vh',
          width: '100vw'
          }"></div>
        </slide>
      </carousel>-->
    <!--</no-ssr>-->

    <swiper :options="swiperOption">
      <swiper-slide v-for="(item, key) of backgrounds" :key="key">
        <div :style="{
          backgroundImage: 'url(' + '/img/' +item.image_url + ')',
          backgroundSize: 'cover',
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'center',
          height: '100vh',
          width: '100vw'
          }"></div>
      </swiper-slide>
      <div class="swiper-pagination" slot="pagination" v-show="(screenWidth > 0 && screenWidth < 768) || !isShowingReview"></div>
    </swiper>

  </div>
</template>

<script>
  import { Carousel, Slide } from 'vue-carousel';
  export default {
    name: "bg-carousel",
    props: {
      backgrounds: {
        type: Array,
        required: false
      }
    },
    components: {
      Carousel,
      Slide
    },
    computed: {
      isShowingReview() {
        return this.$store.getters.isShowingReview;
      },
      screenWidth() {
        return this.$store.getters.getScreenWidth;
      }
    },
    data() {
      return {
        swiperOption: {
          slidesPerView: 1,
          loop: true,
          loopFillGroupWithBlank: true,
          pagination: {
            el: '.swiper-pagination',
            clickable: true
          }
        },
        items: [
          {name: 'background 1', img: "'/img/bg-white.png'"},
          {name: 'background 2', img: "'/img/background-1.png'"},
          {name: 'background 3', img: "'/img/background-2.png'"},
          {name: 'background 4', img: "'/img/bg-white.png'"},
          {name: 'background 5', img: "'/img/background-1.png'"},
        ]
      }
    }
  }
</script>

<style scoped>
</style>
