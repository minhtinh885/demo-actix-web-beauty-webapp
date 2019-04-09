export default function(context) {
  if (!context.store.getters.isAuthenticated) {
    console.log("auth");
    context.redirect("/admin");
  }
}
