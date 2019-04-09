<template>
  <section class="review" v-show="isReview">
    <div class="review-container">
      <div class="feed-back-review show-only-sm">
        <img class="icon icon-large" src="icons/cam-nhan.svg" alt="Cảm nhận khách hàng">
        <span>Cảm nhận khách hàng</span>
      </div>
      <div class="review-indicator hide-sm">
        <a class="indicator-item" @click="toggleLock()">
          <object class="icon icon-xsmall" :data="isLocking === true ? 'icons/lock-solid.svg' : 'icons/lock-open-solid.svg'" type="image/svg+xml"></object>
        </a>
        <a class="indicator-item" @click="changeReview(false)">
          <object class="icon icon-xsmall" data="icons/down.svg" type="image/svg+xml"></object>
        </a>
      </div>

      <swiper :options="swiperOption" ref="mySwiper">
        <swiper-slide v-for="(item, key) of reviews" :key="key">
          <div class="review-item">
            <div class="review-left">
              <a :class="{'review-avatar': true, 'review-avatar-active': isShowing == key}" @click.prevent="showInfo(key)">
                <div class="bg-avatar" :style="{
                backgroundImage: 'url(' + '/img/' +item.image_url + ')',
                backgroundSize: 'cover',
                backgroundRepeat: 'no-repeat',
                backgroundPosition: 'center',
                backgroundOrigin: 'border-box',
                height: windowWidth < 768 ? '100px' : '150px',
                width: windowWidth < 768 ? '100px' : '150px'
                }">
                  <div class="review-info">
                    <h4 class="uppercase noselect">{{ item.fullname }}</h4>
                    <p class="noselect">{{ item.position_at_company }}</p>
                    <p class="noselect">{{ item.phone_number }}</p>
                  </div>
                </div>
              </a>
            </div>
            <div :class="{'review-right': true, 'review-right-hide': isShowing !== key}">
              <p class="noselect">{{ item.content }}</p>
            </div>
          </div>
        </swiper-slide>
        <div class="swiper-pagination" slot="pagination"></div>
      </swiper>
    </div>
  </section>
</template>

<script>
  import { Carousel, Slide } from 'vue-carousel';

  export default {
    name: "reviews",
    props: {
      reviews: {
        type: Array,
        required: false,
      }
    },
    components: {
      Carousel,
      Slide
    },
    computed: {
      isReview() {
        return this.$store.getters.isShowingReview;
      },
      isLocking() {
        return this.$store.getters.isLockingReview;
      },
      swiper() {
        return this.$refs.mySwiper.swiper;
      }

    },
    data() {
      return {
        isFocus: -1,
        windowWidth: 0,
        isShowing: -1,
        swiperOption: {
          // dynamicBullets: true,
          slidesPerView: 'auto',
          centeredSlides: true,
          // watchSlidesProgress: true,
          speed: 500,
          breakpoints: {
            // when window width is <= 320px
            320: {
              slidesPerView: 3,
            },
            375: {
              slidesPerView: 3,
            },
            // when window width is <= 480px
            480: {
              slidesPerView: 4,
            },
            // when window width is <= 640px
            640: {
              slidesPerView: 5,
            },
            700: {
              slidesPerView: 5,
            },
            768: {
              slidesPerView: 5,
            },
            900: {
              slidesPerView: 5,
            },
            1024: {
              slidesPerView: 6,
            },
            1200: {
              slidesPerView: 6,
            },
            1400: {
              slidesPerView: 8,
            },
            1600: {
              slidesPerView: 9,
            },
            1800: {
              slidesPerView: 10,
            },
            2000: {
              slidesPerView: 12,
            }
          },
          pagination: {
            el: '.swiper-pagination',
            clickable: true
          }
        }
      }
    },
    methods: {
      showInfo(index) {
        if (index === this.isShowing) {
          let slideIndex = this.isShowing;
          this.isShowing = -1;
          setTimeout(() => {
            this.swiper.removeSlide([slideIndex + 1, slideIndex + 2]);
            // this.swiper.slideTo(slideIndex, 500, false);
          }, 500);

        } else {
          if (this.isShowing !== -1) {
            let slideIndex = this.isShowing;
            this.isShowing = -1;
            // this.swiper.removeSlide([slideIndex + 1, slideIndex + 2]);
            setTimeout(() => {
              this.swiper.removeSlide([slideIndex + 1, slideIndex + 2]);
              // this.swiper.slideTo(slideIndex, 800, false);
            }, 100);
          }

          setTimeout(() => {
            this.swiper.addSlide(index + 1, [
              '<div class="swiper-slide"></div>',
              '<div class="swiper-slide"></div>'
            ]);
            // this.swiper.slideTo(index + 1, 800, false);
            this.isShowing = index;
          }, 100);
          // this.swiper.addSlide(index + 1, [
          //   '<div class="swiper-slide"></div>',
          //   '<div class="swiper-slide"></div>'
          // ]);
          // this.swiper.slideTo(index + 1, 500, false);

        }
        this.swiper.update();
      },
      changeReview(isReview) {
        if (this.isLocking === false) {
          this.$store.dispatch("changeShowReview", isReview);
        }
      },
      toggleLock() {
        this.$store.dispatch("toggleLockingReview");
      }
    },
    watch: {
      windowWidth(newWidth, oldWidth) {
        this.windowWidth = newWidth;
        if (this.windowWidth < 768) {
          this.$store.dispatch("changeShowReview", true);
        }
        this.$store.dispatch("changeScreenWidth", this.windowWidth);
      }
    },
    mounted() {
      this.swiper.slideTo(Math.floor(this.reviews.length / 2), 0, false);
      this.windowWidth = window.innerWidth;
      this.$nextTick(() => {
        window.addEventListener('resize', () => {
          this.windowWidth = window.innerWidth
        });
      })
    }
  }
</script>

<style scoped></style>
