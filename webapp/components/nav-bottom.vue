<template>
  <nav class="navbar nav-bottom">
    <div class="navbar-items">
      <div class="decorate-15px"></div>
      <nuxt-link to="/" exact-active-class="is-active" class="mr-0">
        <img class="icon-brand" src="icons/logo.png" alt="logo">
        <!--<object class="icon-brand" data="icons/logo.svg" type="image/svg+xml" width="100%" height="100%"></object>-->
      </nuxt-link>
      <div class="decorate-gradient"></div>
      <nuxt-link to="/" exact-active-class="is-active" class="px-10">
        <img class="icon icon-small" src="icons/user.svg" alt="user">
        <!--<object class="icon" data="icons/user.svg" type="image/svg+xml"></object>-->
      </nuxt-link>
      <div class="decorate-user"></div>
    </div>

    <div class="navbar-items">
      <form class="navbar-search" action="#" method="post" @submit.prevent="false">
        <button type="submit" class="search-button"><img class="icon icon-small" src="icons/search.svg" alt="search"></button>
        <input class="search-input" type="text">
      </form>
    </div>

  </nav>
</template>

<script>
    export default {
        name: "nav-bottom"
    }
</script>

<style scoped>
.decorate-15px {
  background-color: rgba(0,0,0,.1);
  width: 15px;
  height: 50px;
}

.decorate-user {
  align-self: center;
  height: 25px;
  width: 4px;
  background-color: #C2C2C2;
  border-radius: 20%;
}
.decorate-gradient {
  width: 15px;
  height: 50px;
  background: rgba(0,0,0,0.1);
  background: -moz-linear-gradient(left, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0) 100%);
  background: -webkit-gradient(left top, right top, color-stop(0%, rgba(0,0,0,0.1)), color-stop(100%, rgba(0,0,0,0)));
  background: -webkit-linear-gradient(left, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0) 100%);
  background: -o-linear-gradient(left, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0) 100%);
  background: -ms-linear-gradient(left, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0) 100%);
  background: linear-gradient(to right, rgba(0,0,0,0.1) 0%, rgba(0,0,0,0) 100%);
}
  .mr-0 {
    margin-right: 0;
  }
  .px-10 {
    padding-left: 20px;
    padding-right: 20px;
  }
</style>
