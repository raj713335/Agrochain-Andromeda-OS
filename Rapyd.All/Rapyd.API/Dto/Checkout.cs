using System.Text.Json.Serialization;

namespace Agrochain.API.Dto
{
    public class CreateCheckoutBody
    {
        [JsonPropertyName("amount")]
        public float Amount { get; set; }

        [JsonPropertyName("country")]
        public string Country { get; set; }

        [JsonPropertyName("currency")]
        public string Currency { get; set; }

        [JsonPropertyName("customer")]
        public string CustomerId { get; set; }

        [JsonPropertyName("complete_payment_url")]
        public string CompletePaymentUrl { get; set; }

        [JsonPropertyName("error_payment_url")]
        public string ErrorPaymentUrl { get; set; }

        [JsonPropertyName("complete_checkout_url")]
        public string CompleteCheckoutUrl { get; set; }
    }

    public class Checkout
    {
        [JsonPropertyName("id")]
        public string Id { get; set; }

        [JsonPropertyName("redirect_url")]
        public string RedirectUrl { get; set; }
    }
}