global.File= typeof window === 'undefined' ? Object : window.File
const pkg = require('./package')

module.exports = {
  mode: 'universal',

  /*
  ** Headers of the page
  */
  head: {
    title: "Nano Collagen",
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: "Nano Collagen" }
    ],
    link: [
      { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
      { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css?family=Oswald:300,400&amp;subset=vietnamese'},
    ]
  },

  env: {
    baseUrl: process.env.BASE_URL || 'http://localhost:3001'
  },

  /*
  ** Customize the progress-bar color
  */
  loading: { color: '#00FFFF' },

  /*
  ** Global CSS
  */
  css: [
    // 'swiper/dist/css/swiper.css',
    // '~assets/styles/main.css',
  ],

  /*
  ** Plugins to load before mounting the App
  */
  plugins: [
    { src: '~/plugins/core-components.js', ssr: true },
    { src: '~/plugins/nossr-components.js', ssr: false },
    { src: '~/plugins/nuxt-swiper-plugin.js', ssr: false },
  ],

  /*
  ** Nuxt.js modules
  */
  modules: [
    // Doc: https://github.com/nuxt-community/axios-module#usage
    '@nuxtjs/axios',
    ['nuxt-buefy', { css: false, materialDesignIcons: true }],
  ],
  /*
  ** Axios module configuration
  */
  axios: {
    // See https://github.com/nuxt-community/axios-module#options
  },

  /*
  ** Build configuration
  */
  build: {
    /*
    ** You can extend webpack config here
    */
    extend(config, ctx) {
      
    }
  }
}
