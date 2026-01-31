<template>
  <PurchaseModal
    ref="purchaseModal"
    :product="midasProduct"
    :country="country"
    :publishable-key="config.public.stripePublishableKey"
    :send-billing-request="
      async (body) =>
        await useBaseFetch('billing/payment', { internal: true, method: 'POST', body })
    "
    :fetch-payment-data="fetchPaymentData"
    :on-error="
      (err) =>
        data.$notify({
          group: 'main',
          title: 'An error occurred',
          type: 'error',
          text: err.message ?? (err.data ? err.data.description : err),
        })
    "
    :customer="customer"
    :payment-methods="paymentMethods"
    :return-url="`${config.public.siteUrl}/settings/billing`"
  />
</template>
<script setup>
import { HeartIcon, SparklesIcon, StarIcon, SettingsIcon } from "@nml/assets";
import { PurchaseModal } from "@nml/ui";
import { calculateSavings, formatPrice, getCurrency } from "@nml/utils";
import { products } from "~/generated/state.json";

useHead({
  script: [
    {
      src: "https://js.stripe.com/v3/",
      defer: true,
      async: true,
    },
  ],
});

const vintl = useVIntl();

const data = useNuxtApp();
const config = useRuntimeConfig();

const auth = await useAuth();
const purchaseModal = ref();
const midasProduct = ref(products.find((x) => x.metadata.type === "midas"));
const country = useUserCountry();
const price = computed(() =>
  midasProduct.value.prices.find((x) => x.currency_code === getCurrency(country.value)),
);
const customer = ref();
const paymentMethods = ref([]);

async function fetchPaymentData() {
  [customer.value, paymentMethods.value] = await Promise.all([
    useBaseFetch("billing/customer", { internal: true }),
    useBaseFetch("billing/payment_methods", { internal: true }),
  ]);
}

const route = useRoute();
onMounted(() => {
  if (route.query.showModal) {
    purchaseModal.value.show();
  }
});
</script>
<style lang="scss" scoped>
.main-hero {
  background: linear-gradient(360deg, rgba(199, 138, 255, 0.2) 10.92%, var(--color-bg) 100%),
    var(--color-accent-contrast);
  margin-top: -5rem;
  padding: 11.25rem 1rem 8rem;

  display: flex;
  align-items: center;
  flex-direction: column;
}

.perks-hero {
  background-color: var(--color-accent-contrast);
  display: flex;
  align-items: center;
  flex-direction: column;
  padding: 4rem 1rem;
}
</style>
