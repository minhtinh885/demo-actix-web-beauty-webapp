<template>
  <div style="margin-top: 120px;">
    <div class="container">
      <a class="button is-primary" @click.prevent="showBackgroundNewModal()">Thêm hình ảnh</a>
      <a class="button is-danger" @click.prevent="reload()">Tải lại</a>
    </div>

    <div class="container">
      <div class="flex-container" ref="flexContainer">
        <div class="item" v-for="(item, index) of backgrounds" :key="index" :style="{backgroundImage: 'url(' + '/img/' + item.image_url + ')'}">
          <a class="item-button button is-primary is-outlined" @click.prevent="showBackgroundOpen(item)">Xem ảnh</a>
          <a class="delete" @click.prevent="toggleModalBackground(item)"></a>
          <a class="item-icon repair" @click.prevent="showBackgroundEditModal(item)"><b-icon icon="settings" size="is-small"></b-icon></a>
          <a class="item-icon eye" @click.prevent="toggleBackgroundStatus(item.id, item.status)"><b-icon :icon="item.status === 0 ? 'eye-off-outline': 'eye-outline'" size="is-small"></b-icon></a>
        </div>
      </div>
    </div>

    <b-modal :active.sync="backgroundViewOpen.isBackgroundViewOpen" class="text-white">
      <small>{{backgroundViewOpen.selectedBackground.image_url}}</small> - <small>({{backgroundViewOpen.selectedBackground.alt}})</small>
      <p class="image is-4by3">
        <img :src="'/img/' + backgroundViewOpen.selectedBackground.image_url">
      </p>
    </b-modal>

    <b-modal :active.sync="backgroundModal.isModalBackgroundOpen" :width="640" scroll="keep" :canCancel="!this.backgroundModal.submitBackgroundLoading">
      <div class="modal-card">
        <section class="modal-card-body">
          <h1>
            <b-icon icon="help-circle"></b-icon>
            Có muốn xoá?
            <span class="tag is-dark">{{backgroundModal.selectedBackground.image_url}}</span>
          </h1>
        </section>
        <footer class="modal-card-foot">
          <div class="align-right">
            <button :class="{button: true, 'is-danger': true, 'is-loading': backgroundModal.submitBackgroundLoading}"
                    @click="removeBackground()">
              <b-icon icon="check-circle"></b-icon>
              <span>Xoá</span></button>
            <button class="button" @click="closeModalBackground()" :disabled="backgroundModal.submitBackgroundLoading">Không
            </button>
          </div>
        </footer>
      </div>
    </b-modal>

    <b-modal :active.sync="backgroundEditModal.isModalBackgroundEditOpen" :width="640" scroll="keep">
      <div class="modal-card">
        <header class="modal-card-head">
          <p class="modal-card-title">{{ background.id === -1 ? "Tạo mới" : "Cập nhật"}}</p>
        </header>
        <section class="modal-card-body">
          <b-field label="Tên hình ảnh">
            <b-input v-model="background.image_url" disabled></b-input>
          </b-field>

          <b-field label="Alt">
            <b-input v-model="background.alt" type="text" maxlength="255">
            </b-input>
          </b-field>

          <b-field>
            <b-radio v-model="background.status" native-value="1">
              Hiển thị
            </b-radio>
            <b-radio v-model="background.status" native-value="0">
              Không hiển thị
            </b-radio>
          </b-field>

          <b-field class="file">
            <p class="image is-64x64">
              <img v-if="background.image_url !== '' && background.image_url !== undefined" :src="'/img/' + background.image_url">
            </p>

            <b-upload v-model="file" v-if="background.id === -1">
              <a class="button is-primary">
                <b-icon icon="upload"></b-icon>
                <span>Click to upload</span>
              </a>
            </b-upload>
            <span class="file-name" v-if="file">
            {{ file.name }}
        </span>
          </b-field>
        </section>
        <footer class="modal-card-foot">
          <div class="align-right">
            <button :class="{button: true, 'is-danger': true, 'is-loading': backgroundEditModal.submitBackgroundEditLoading}"
                    @click="submitEditBackground()">
              <b-icon icon="check-circle"></b-icon>
              <span>Đồng ý</span></button>
            <button class="button" @click="closeModalEditBackground()" :disabled="backgroundEditModal.submitBackgroundEditLoading">Không
            </button>
          </div>

        </footer>
      </div>
    </b-modal>
  </div>
</template>

<script>

  export default {
    name: "anh-nen",
    layout: 'admin',
    middleware: ['check-auth', 'auth'],
    computed: {
      backgrounds() {
        return this.$store.getters.getBackgrounds;
      }
    },
    data() {
      return {
        file: null,
        background: {
          id: -1,
          image_url: '',
          alt: '',
          status: 1,
        },
        backgroundViewOpen: {
          isBackgroundViewOpen: false,
          selectedBackground: {}
        },
        backgroundModal: {
          isModalBackgroundOpen: false,
          selectedBackground: {},
          submitBackgroundLoading: false,
        },
        backgroundEditModal: {
          isModalBackgroundEditOpen: false,
          selectedBackgroundEditId: -1,
          submitBackgroundEditLoading: false,
        }

      }
    },
    watch: {
      file: function (newVal) {
        this.background.image_url = newVal.name;
      }
    },
    methods: {
      reload() {
        if (window !== undefined) {
          window.location.reload();
        }
      },
      submitEditBackground() {
        this.$store.dispatch("SubmitBackgroundForm", {background: this.background, file: this.file});
      },
      showBackgroundOpen(background) {
        this.backgroundViewOpen.isBackgroundViewOpen = true;
        this.backgroundViewOpen.selectedBackground = background;
      },
      showBackgroundNewModal() {
        this.backgroundEditModal.isModalBackgroundEditOpen = true;
        this.background = {
          id: -1,
          image_url: '',
          alt: '',
          status: 1,
        }
      },
      showBackgroundEditModal(background) {
        this.backgroundEditModal.isModalBackgroundEditOpen = true;
        this.background = background;
      },
      closeModalEditBackground() {
        this.backgroundEditModal.isModalBackgroundEditOpen = false;
      },

      // REMOVE BACKGROUND MODAL
      closeModalBackground() {
        if (!this.backgroundModal.submitBackgroundLoading) {
          this.backgroundModal.selectedBackground = {};
          this.backgroundModal.isModalBackgroundOpen = false;
        }
      },
      toggleModalBackground(selectedBackground) {
        this.backgroundModal.selectedBackground = selectedBackground;
        this.backgroundModal.isModalBackgroundOpen = !this.isModalBackgroundOpen;
        this.backgroundModal.submitBackgroundLoading = false;
      },
      removeBackground() {
        if (Object.keys(this.backgroundModal.selectedBackground).length !== 0) {
          this.backgroundModal.submitBackgroundLoading = true;
          this.$store.dispatch("removeBackground", this.backgroundModal.selectedBackground.id).then(response => {
            this.backgroundModal.submitBackgroundLoading = false;
            this.closeModalBackground();
          }, error => {
            console.log(error);
            this.backgroundModal.submitBackgroundLoading = false;
            this.closeModalBackground();
          });
        }
      },
      toggleBackgroundStatus(id, _status) {
        let status = 0;
        if (_status === 0) {
          status = 1;
        }
        let payload = {
          id: id,
          status: status,
        };
        this.$store.dispatch("changeBackgroundStatus", payload);
      },
    },
    mounted() {
      return Promise.all([
        this.$store.dispatch("fetchBackgrounds"),
      ]);
    },

    asyncData(context) {
      return Promise.all([
        context.app.store.dispatch("fetchBackgrounds"),
      ]);
    }
  }
</script>

<style scoped>
  .flex-container {
    margin-top: 50px;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-around;
  }
  .item {
    min-width: 200px;
    min-height: 200px;
    overflow: hidden;
    margin-left: 5px;
    margin-right: 5px;
    flex: 1;
    position: relative;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
  }
  .item-button {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-weight: 900;
    display: none;
  }
  .delete {
    position: absolute;
    right: 2px;
    top: 2px;
    display: none;
  }
  .item-icon {
    cursor: pointer;
    height: 20px;
    outline: none;
    position: absolute;
    width: 20px;
    color: #fff;
    display: none;
  }

  .repair {
    right: 0px;
    top: 28px;
  }

  .eye {
    right: 0px;
    top: 55px;
  }

  .item:hover::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, .3);
  }
  .item:hover .item-button, .item:hover .item-icon {
    display: block;
  }

  .item:hover .delete {
    display: block;
  }



  .item img {
    height: 200px;
    width: auto;
  }
  .group-action-button {
    display: flex;
    align-items: center;
    justify-content: space-around;
  }

  .align-right {
    margin-left: auto;
  }
  
  .text-white {
    color: #fff;
  }

</style>
