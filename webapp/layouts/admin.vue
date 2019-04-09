<template>
  <div>
    <nav class="navbar is-transparent is-fixed-top is-dark" role="navigation" aria-label="main navigation">
      <div class="container">
        <div class="navbar-brand">
          <nuxt-link to="/admin" exact-active-class="is-active" class="navbar-item">
            <img src="/icons/logo.png" width="112" height="28">
          </nuxt-link>
          <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" @click.prevent="toggleNavbar()">
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div id="navbarBasicExample" :class="{'navbar-menu': true, 'is-active': isNavOpen}">
          <div class="navbar-start">
            <nuxt-link to="/admin/background" exact-active-class="is-active" class="navbar-item" v-if="isAuthenticated">
              Ảnh nền
            </nuxt-link>
            <nuxt-link to="/admin/review" exact-active-class="is-active" class="navbar-item" v-if="isAuthenticated">
              Đánh giá
            </nuxt-link>
          </div>
          <div class="navbar-end">
            <div class="navbar-item" v-if="isAuthenticated">
              <a class="button is-primary" @click="logout()">
                <strong>Đăng xuất</strong>
              </a>
            </div>
            <div class="navbar-item" v-if="!isAuthenticated">
              <div class="buttons">
                <a class="button is-primary" disabled>
                  <strong>Đăng ký</strong>
                </a>
                <a class="button is-light" @click="login()">
                  Đăng nhập
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>
    <main class="container">
      <nuxt/>
      <b-modal :active.sync="loginModal.isLoginModal">
        <div class="modal-card" style="width: auto">
          <header class="modal-card-head">
            <p class="modal-card-title">Đăng nhập</p>
          </header>
          <section class="modal-card-body">
            <b-field label="Email">
              <b-input
                type="email"
                :value="email"
                v-model="email"
                placeholder="Tài khoản email"
                required>
              </b-input>
            </b-field>

            <b-field label="Mật khẩu">
              <b-input
                type="password"
                :value="password"
                v-model="password"
                password-reveal
                placeholder="Mật khẩu"
                required>
              </b-input>
            </b-field>

          </section>
          <footer class="modal-card-foot">
            <button class="button" type="button" @click="loginModal.isLoginModal = false;">Đóng</button>
            <button :class="{button: true, 'is-danger': true, 'is-loading': loginModal.submitLoading}" @click="submitLogin()" :disabled="loginModal.submitLoading">Đăng nhập</button>
          </footer>
        </div>
      </b-modal>
    </main>
  </div>
</template>
<script>
  export default {
    head() {
      return {
        link: [
          { rel: 'stylesheet', href: 'https://unpkg.com/buefy/dist/buefy.min.css' }
        ]
      }
    },
    computed: {
      isAuthenticated: function () {
        return this.$store.getters.isAuthenticated;
      }
    },
    data() {
      return {
        isNavOpen: false,
        email: '',
        password: '',
        loginModal: {
          isLoginModal: false,
          submitLoading: false,
        }
      }
    },
    methods: {
      submitLogin() {
        if (this.email.length > 5 && this.password.length > 5) {
          this.loginModal.submitLoading = true;
          this.$store.dispatch("authenticateUser", {email: this.email, password: this.password}).then(res => {
            this.loginModal.submitLoading = false;
            this.loginModal.isLoginModal = false;
            this.email = '';
            this.password = '';
          })
        }
      },
      toggleNavbar() {
        this.isNavOpen = !this.isNavOpen;
      },
      logout() {
        this.$store.dispatch("logout");
      },
      login() {
        this.loginModal.isLoginModal = true;
      }
    },
    mounted() {
      this.$store.dispatch("initAuth");
      this.loginModal.isLoginModal = !this.$store.getters.isAuthenticated;
    }

  }
</script>
<style>

</style>
