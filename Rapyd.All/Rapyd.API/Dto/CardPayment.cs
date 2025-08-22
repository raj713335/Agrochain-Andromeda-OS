using System.Text.Json.Serialization;

namespace Agrochain.API.Dto
{
    public class CardPayment
    {
        [JsonPropertyName("amount")]
        public int amount { get; set; }

        [JsonPropertyName("currency")]
        public string currency { get; set; }

        [JsonPropertyName("customer")]
        public string customer { get; set; }

        [JsonPropertyName("payment_method")]
        public PaymentMethod payment_method { get; set; }

        [JsonPropertyName("capture")]
        public bool capture { get; set; }
    }
}
