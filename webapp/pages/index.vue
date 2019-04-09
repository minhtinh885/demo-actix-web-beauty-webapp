<template>
  <section class="container">

  </section>
</template>

<script>

export default {
  components: {
  },
  asyncData(context) {
    return Promise.all([
      context.store.dispatch("fetchImages"),
      context.store.dispatch("fetchReviews")
    ]);
  }
}
</script>

<style>
</style>
