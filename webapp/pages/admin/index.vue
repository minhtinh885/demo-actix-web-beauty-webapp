<template>
  <div style="margin-top: 120px;">
    <h1 v-if="!isAuthenticated" class="title-alert">Yêu cầu đăng nhập!</h1>
  </div>
</template>

<script>

  export default {
    name: "index",
    layout: 'admin',
    computed: {
      isAuthenticated: function () {
        return this.$store.getters.isAuthenticated;
      }
    },
    data() {
      return {

      }
    },
    methods: {

    },
    mounted() {

    }
  }
</script>

<style scoped>
.title-alert {
  text-align: center;
  font-size: 5rem;
  text-transform: uppercase;
}
</style>
