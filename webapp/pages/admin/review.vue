<template>
  <div style="margin-top: 120px;">
    <div class="container">
      <a class="button is-primary" @click.prevent="showReviewNewModal()">Tạo mới đánh giá</a>
    </div>
    <b-table
      :data="reviews"
      paginated
      per-page="5"
      :opened-detailed="defaultOpenedDetails"
      detailed
      detail-key="id"
      @details-open="(row, index) => $toast.open(`Đã mở ${row.fullname}`)"
    >

      <template slot-scope="props">
        <b-table-column field="id" label="ID" width="40" numeric>
          {{ props.row.id }}
        </b-table-column>

        <b-table-column field="fullname" label="Tên đầy đủ" sortable>
          {{ props.row.fullname }}
        </b-table-column>

        <b-table-column field="position_at_company" label="Chức vụ" sortable>
          {{ props.row.position_at_company }}
        </b-table-column>

        <b-table-column field="created_at" label="Ngày tạo" sortable centered>
                <span class="tag is-success">
                    {{ new Date(props.row.created_at).toLocaleDateString('vi-VN') }}
                </span>
        </b-table-column>

        <b-table-column label="Giới tính">
          <b-icon
            :icon="props.row.gender === 1 ? 'gender-male' : 'gender-female'">
          </b-icon>
          {{ props.row.gender === 1 ? 'Nam' : 'Nữ' }}
        </b-table-column>

        <b-table-column field="status" label="Hiển thị" sortable centered>
          <a
            :class="{button: true, 'is-small': true, 'is-info': props.row.status === 1, 'is-danger': props.row.status === 0 }"
            @click.prevent="toggleReviewStatus(props.row.id, props.row.status)">
            {{ props.row.status === 1 ? 'Đang hiển thị' : 'Không' }}
          </a>
        </b-table-column>
        <b-table-column field="action" label="*" centered>
          <div class="group-action-button">
            <a class="button is-small is-warning" @click.prevent="showReviewEditModal(props.row.id)">
              <b-icon icon="pencil"></b-icon>
            </a>
            <a class="delete is-danger" @click.prevent="toggleModalReview(props.row.id, props.row.fullname)"></a>
          </div>
        </b-table-column>
      </template>

      <template slot="detail" slot-scope="props">
        <article class="media">
          <figure class="media-left">
            <p class="image is-64x64">
              <img :src="'/img/' + props.row.image_url">
            </p>
          </figure>
          <div class="media-content">
            <div class="content">
              <p>
                <strong>{{ props.row.fullname }}</strong>
                <small>@{{ props.row.phone_number }}</small>
                <!--<small>31m</small>-->
                <br>
                {{ props.row.content }}
              </p>
            </div>
          </div>
        </article>
      </template>
    </b-table>
    <b-modal :active.sync="reviewModal.isModalReviewOpen" :width="640" scroll="keep" :canCancel="!this.reviewModal.submitReviewLoading">
      <div class="modal-card">
        <section class="modal-card-body">
          <h1>
            <b-icon icon="help-circle"></b-icon>
            Có muốn xoá?
            <span class="tag is-dark">{{reviewModal.selectedReviewName}}</span>
          </h1>
        </section>
        <footer class="modal-card-foot">
          <div class="align-right">
            <button :class="{button: true, 'is-danger': true, 'is-loading': reviewModal.submitReviewLoading}"
                    @click="removeReview()">
              <b-icon icon="check-circle"></b-icon>
              <span>Xoá</span></button>
            <button class="button" @click="closeModalReview()" :disabled="reviewModal.submitReviewLoading">Không
            </button>
          </div>
        </footer>
      </div>
    </b-modal>
    <!--<div :class="{modal: true, 'is-active': reviewModal.isModalReviewOpen}">
      <div class="modal-background" @click="closeModalReview()"></div>
      <div class="modal-card">
        <section class="modal-card-body">
          <h1>
            <b-icon icon="help-circle"></b-icon>
            Có muốn xoá?
            <span class="tag is-dark">{{reviewModal.selectedReviewName}}</span>
          </h1>
        </section>
        <footer class="modal-card-foot">
          <div class="align-right">
            <button :class="{button: true, 'is-danger': true, 'is-loading': reviewModal.submitReviewLoading}"
                    @click="removeReview()">
              <b-icon icon="check-circle"></b-icon>
              <span>Xoá</span></button>
            <button class="button" @click="closeModalReview()" :disabled="reviewModal.submitReviewLoading">Không
            </button>
          </div>
        </footer>
      </div>
      <button class="modal-close is-large" @click="closeModalReview()" aria-label="close"></button>
    </div>-->

    <b-modal :active.sync="reviewEditModal.isModalReviewEditOpen" :width="640" scroll="keep">
      <div class="modal-card">
        <header class="modal-card-head">
          <p class="modal-card-title">{{ review.id === -1 ? "Tạo mới đánh giá" : "Cập nhật đánh giá"}}</p>
        </header>
        <section class="modal-card-body">
          <b-field label="Tên">
            <b-input v-model="review.fullname"></b-input>
          </b-field>
          <b-field>
            <b-radio v-model="review.gender"
                     native-value="1">
              Nam
            </b-radio>
            <b-radio v-model="review.gender"
                     native-value="0">
              Nữ
            </b-radio>
          </b-field>

          <b-field label="Số điện thoại">
            <b-input v-model="review.phone_number" type="text" maxlength="20">
            </b-input>
          </b-field>

          <b-field label="Công việc">
            <b-input v-model="review.position_at_company" maxlength="255"></b-input>
          </b-field>

          <b-field>
            <b-radio v-model="review.status" native-value="1">
              Hiển thị
            </b-radio>
            <b-radio v-model="review.status" native-value="0">
              Không hiển thị
            </b-radio>
          </b-field>

          <b-field label="Nội dung">
            <b-input v-model="review.content" maxlength="255" type="textarea"></b-input>
          </b-field>

          <b-field class="file">
            <p class="image is-64x64">
              <img v-if="review.image_url !== '' && review.image_url !== undefined" :src="'/img/' + review.image_url">
            </p>

            <b-upload v-model="file">
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
            <button :class="{button: true, 'is-danger': true, 'is-loading': reviewModal.submitReviewLoading}"
                    @click="submitEditReview()">
              <b-icon icon="check-circle"></b-icon>
              <span>Đồng ý</span></button>
            <button class="button" @click="closeModalEditReview()" :disabled="reviewModal.submitReviewLoading">Không
            </button>
          </div>

        </footer>
      </div>
    </b-modal>
  </div>
</template>

<script>

  export default {
    name: "danh-gia",
    layout: 'admin',
    middleware: ['check-auth', 'auth'],
    computed: {
      reviews() {
        return this.$store.getters.getFullReviews;
      }
    },
    data() {
      return {
        file: null,
        review: {
          id: -1,
          fullname: '',
          gender: 1,
          phone_number: '',
          position_at_company: '',
          status: 1,
          content: '',
        },
        defaultOpenedDetails: [1],
        reviewModal: {
          isModalReviewOpen: false,
          selectedReviewId: -1,
          selectedReviewName: '',
          submitReviewLoading: false,
        },
        reviewEditModal: {
          isModalReviewEditOpen: false,
          selectedReviewEditId: -1,
          submitReviewEditLoading: false,
        }

      }
    },
    methods: {
      submitEditReview() {
        this.$store.dispatch("SubmitReviewForm", {review: this.review, file: this.file});

      },
      showReviewNewModal() {
        this.reviewEditModal.isModalReviewEditOpen = true;
        this.review = {
          id: -1,
          fullname: '',
          gender: 1,
          phone_number: '',
          position_at_company: '',
          status: 1,
          content: '',
          image_url: '',
        }
      },
      showReviewEditModal(_id) {
        this.reviewEditModal.isModalReviewEditOpen = true;
        let index = this.reviews.findIndex(x => x.id === _id);
        this.review = {
          id: _id,
          fullname: this.reviews[index].fullname,
          gender: this.reviews[index].gender,
          phone_number: this.reviews[index].phone_number,
          position_at_company: this.reviews[index].position_at_company,
          status: this.reviews[index].status,
          content: this.reviews[index].content,
          image_url: this.reviews[index].image_url,
        }
      },
      closeModalEditReview() {
        this.reviewEditModal.isModalReviewEditOpen = false;
      },

      // REMOVE REVIEW MODAL
      closeModalReview() {
        if (!this.reviewModal.submitReviewLoading) {
          this.reviewModal.selectedReviewId = -1;
          this.reviewModal.selectedReviewName = '';
          this.reviewModal.isModalReviewOpen = false;
        }
      },
      toggleModalReview(id, fullname) {
        this.reviewModal.selectedReviewName = fullname;
        this.reviewModal.selectedReviewId = id;
        this.reviewModal.isModalReviewOpen = !this.isModalReviewOpen;
        this.reviewModal.submitReviewLoading = false;
      },
      removeReview() {
        if (this.reviewModal.selectedReviewId !== -1) {
          this.reviewModal.submitReviewLoading = true;
          this.$store.dispatch("removeReview", this.reviewModal.selectedReviewId).then(response => {
            this.reviewModal.submitReviewLoading = false;
            this.closeModalReview();
          }, error => {
            console.log(error);
            this.reviewModal.submitReviewLoading = false;
            this.closeModalReview();
          });
        }
      },
      toggleReviewStatus(id, _status) {
        let status = 0;
        if (_status === 0) {
          status = 1;
        }
        let payload = {
          id: id,
          status: status,
        };
        this.$store.dispatch("changeReviewStatus", payload);
      },
    },
    mounted() {
      return Promise.all([
        this.$store.dispatch("fetchFullReviews")
      ]);
    },
    asyncData(context) {
      return Promise.all([
        context.app.store.dispatch("fetchFullReviews"),
      ]);
    }
  }
</script>

<style scoped>
  .group-action-button {
    display: flex;
    align-items: center;
    justify-content: space-around;
  }

  .align-right {
    margin-left: auto;
  }

</style>
