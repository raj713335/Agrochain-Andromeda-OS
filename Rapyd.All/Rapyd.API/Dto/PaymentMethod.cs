using System.Text.Json.Serialization;

namespace Agrochain.API.Dto
{
    public class PaymentMethod
    {
        [JsonPropertyName("type")]
        public string type { get; set; }

        [JsonPropertyName("fields")]
        public Fields fields { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }
    }

    public class Fields
    {
        [JsonPropertyName("number")]
        public string number { get; set; }

        [JsonPropertyName("expiration_month")]
        public string expiration_month { get; set; }

        [JsonPropertyName("expiration_year")]
        public string expiration_year { get; set; }

        [JsonPropertyName("name")]
        public string name { get; set; }

        [JsonPropertyName("cvv")]
        public string cvv { get; set; }
    }
}