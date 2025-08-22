using System.Text.Json.Serialization;

namespace Agrochain.API.Dto.Response
{
    public class Datum
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("ewallet_contact")]
        public EwalletContact ewallet_contact { get; set; }

        [JsonPropertyName("status")]
        public string status { get; set; }

        [JsonPropertyName("card_id")]
        public string card_id { get; set; }

        [JsonPropertyName("assigned_at")]
        public int assigned_at { get; set; }

        [JsonPropertyName("activated_at")]
        public int activated_at { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("country_iso_alpha_2")]
        public string country_iso_alpha_2 { get; set; }

        [JsonPropertyName("created_at")]
        public int created_at { get; set; }

        [JsonPropertyName("blocked_reason")]
        public string blocked_reason { get; set; }

        [JsonPropertyName("card_program")]
        public string card_program { get; set; }

        [JsonPropertyName("card_number")]
        public string card_number { get; set; }

        [JsonPropertyName("cvv")]
        public string cvv { get; set; }

        [JsonPropertyName("expiration_month")]
        public string expiration_month { get; set; }

        [JsonPropertyName("expiration_year")]
        public string expiration_year { get; set; }

        [JsonPropertyName("bin")]
        public string bin { get; set; }

        [JsonPropertyName("sub_bin")]
        public string sub_bin { get; set; }
    }

    public class ListOfIssuedCards
    {
        [JsonPropertyName("status")]
        public Status status { get; set; }

        [JsonPropertyName("data")]
        public List<Datum> data { get; set; }
    }
}
