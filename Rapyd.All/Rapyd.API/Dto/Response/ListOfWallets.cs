using System.Text.Json.Serialization;

namespace Agrochain.API.Dto.Response
{
    public class Account
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("currency")]
        public string currency { get; set; }

        [JsonPropertyName("alias")]
        public string alias { get; set; }

        [JsonPropertyName("balance")]
        public int balance { get; set; }

        [JsonPropertyName("received_balance")]
        public int received_balance { get; set; }

        [JsonPropertyName("on_hold_balance")]
        public int on_hold_balance { get; set; }

        [JsonPropertyName("reserve_balance")]
        public int reserve_balance { get; set; }

        [JsonPropertyName("limits")]
        public object limits { get; set; }

        [JsonPropertyName("limit")]
        public object limit { get; set; }
    }

    public class Wallet
    {
        [JsonPropertyName("phone_number")]
        public string phone_number { get; set; }

        [JsonPropertyName("email")]
        public object email { get; set; }

        [JsonPropertyName("first_name")]
        public string first_name { get; set; }

        [JsonPropertyName("last_name")]
        public string last_name { get; set; }

        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("status")]
        public string status { get; set; }

        [JsonPropertyName("accounts")]
        public List<Account> accounts { get; set; }

        [JsonPropertyName("verification_status")]
        public string verification_status { get; set; }

        [JsonPropertyName("type")]
        public string type { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("ewallet_reference_id")]
        public object ewallet_reference_id { get; set; }

        [JsonPropertyName("category")]
        public object category { get; set; }
    }

    public class ListOfWallet
    {
        [JsonPropertyName("status")]
        public Status status { get; set; }

        [JsonPropertyName("data")]
        public List<Wallet> data { get; set; }
    }
}
