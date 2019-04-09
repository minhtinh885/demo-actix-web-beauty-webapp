<template>
    <p> </p>
</template>

<script>
    export default {
        name: "chinh-sach-dai-ly",
      asyncData(context) {
        return Promise.all([
          context.store.dispatch("fetchImages"),
          context.store.dispatch("fetchReviews")
        ]);
      }
    }
</script>

<style scoped>

</style>
