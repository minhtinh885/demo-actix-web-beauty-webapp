<template>
    <ul class="menu-list">
        <nuxt-link to="/" exact-active-class="is-active" class="hide-sm">
          <li><span>Trang chủ</span></li>
        </nuxt-link>
        <nuxt-link v-for="(item, key) of items" :to="item.to" exact-active-class="is-active" :key="key">
          <li>
            <!--<span><object class="icon icon-large" :data="item.icon" type="image/svg+xml"></object></span>-->
            <img class="icon icon-large icon-nav" :src="item.icon" :alt="item.title">
            <span class="hide-sm">{{ item.title }}</span>
          </li>
        </nuxt-link>
    </ul>
</template>

<script>
    export default {
        name: "navigation",
      data() {
          return {
            items: [
              { title: 'Giới thiệu', icon: 'icons/gioi-thieu.svg', to: { name: 'gioi-thieu'}},
              { title: 'Chính sách đại lý', icon: 'icons/chinh-sach.svg', to: { name: 'chinh-sach-dai-ly'}},
              { title: 'Liên hệ', icon: 'icons/lien-he.svg', to: { name: 'lien-he'}},
            ]
          }
      }
    }
</script>

<style scoped>
</style>
