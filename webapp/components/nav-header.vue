<template>
  <nav class="navbar nav-top">
    <nuxt-link to="/" exact-active-class="is-active">
      <img class="icon icon" src="icons/user.svg" alt="user">
      <!--<object class="icon icon-small" data="icons/user.svg" type="image/svg+xml"></object>-->
    </nuxt-link>
    <nuxt-link to="/" exact-active-class="is-active">
      <img class="icon-brand" src="icons/logo.png" alt="logo">
      <!--<object class="icon-brand" data="icons/logo.svg" type="image/svg+xml" width="100%" height="100%"></object>-->
    </nuxt-link>
    <nuxt-link to="/" exact-active-class="is-active">
      <img class="icon" src="icons/search.svg" alt="search">
      <!--<object class="icon icon-small" data="icons/search.svg" type="image/svg+xml"></object>-->
    </nuxt-link>
  </nav>
</template>

<script>
    export default {
        name: "nav-header"
    }
</script>

<style scoped>

</style>
