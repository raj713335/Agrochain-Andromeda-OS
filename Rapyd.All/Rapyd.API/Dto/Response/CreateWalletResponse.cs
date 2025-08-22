using System.Text.Json.Serialization;

namespace Agrochain.API.Dto.Response
{
    public class AddressForWallet
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("name")]
        public string name { get; set; }

        [JsonPropertyName("line_1")]
        public string line_1 { get; set; }

        [JsonPropertyName("line_2")]
        public string line_2 { get; set; }

        [JsonPropertyName("line_3")]
        public string line_3 { get; set; }

        [JsonPropertyName("city")]
        public string city { get; set; }

        [JsonPropertyName("state")]
        public string state { get; set; }

        [JsonPropertyName("country")]
        public string country { get; set; }

        [JsonPropertyName("zip")]
        public string zip { get; set; }

        [JsonPropertyName("phone_number")]
        public string phone_number { get; set; }

        [JsonPropertyName("canton")]
        public string canton { get; set; }

        [JsonPropertyName("district")]
        public string district { get; set; }

        [JsonPropertyName("created_at")]
        public int created_at { get; set; }
    }

    public class ContactsForWallet
    {
        [JsonPropertyName("data")]
        public List<Data> data { get; set; }

        [JsonPropertyName("has_more")]
        public bool has_more { get; set; }

        [JsonPropertyName("total_count")]
        public int total_count { get; set; }

        [JsonPropertyName("url")]
        public string url { get; set; }
    }

    public class Data
    {
        [JsonPropertyName("phone_number")]
        public string phone_number { get; set; }

        [JsonPropertyName("email")]
        public string email { get; set; }

        [JsonPropertyName("first_name")]
        public string first_name { get; set; }

        [JsonPropertyName("last_name")]
        public string last_name { get; set; }

        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("status")]
        public string status { get; set; }

        [JsonPropertyName("accounts")]
        public List<object> accounts { get; set; }

        [JsonPropertyName("verification_status")]
        public string verification_status { get; set; }

        [JsonPropertyName("type")]
        public string type { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("ewallet_reference_id")]
        public string ewallet_reference_id { get; set; }

        [JsonPropertyName("category")]
        public object category { get; set; }

        [JsonPropertyName("contacts")]
        public ContactsForWallet contacts { get; set; }

        [JsonPropertyName("middle_name")]
        public string middle_name { get; set; }

        [JsonPropertyName("second_last_name")]
        public string second_last_name { get; set; }

        [JsonPropertyName("gender")]
        public string gender { get; set; }

        [JsonPropertyName("marital_status")]
        public string marital_status { get; set; }

        [JsonPropertyName("house_type")]
        public string house_type { get; set; }

        [JsonPropertyName("contact_type")]
        public string contact_type { get; set; }

        [JsonPropertyName("identification_type")]
        public string identification_type { get; set; }

        [JsonPropertyName("identification_number")]
        public string identification_number { get; set; }

        [JsonPropertyName("issued_card_data")]
        public IssuedCardData issued_card_data { get; set; }

        [JsonPropertyName("date_of_birth")]
        public string date_of_birth { get; set; }

        [JsonPropertyName("country")]
        public string country { get; set; }

        [JsonPropertyName("nationality")]
        public string nationality { get; set; }

        [JsonPropertyName("address")]
        public AddressForWallet address { get; set; }

        [JsonPropertyName("ewallet")]
        public string ewallet { get; set; }

        [JsonPropertyName("created_at")]
        public int created_at { get; set; }

        [JsonPropertyName("business_details")]
        public object business_details { get; set; }

        [JsonPropertyName("compliance_profile")]
        public int compliance_profile { get; set; }

        [JsonPropertyName("send_notifications")]
        public bool send_notifications { get; set; }

        [JsonPropertyName("mothers_name")]
        public string mothers_name { get; set; }
    }

    public class IssuedCardData
    {
        [JsonPropertyName("preferred_name")]
        public string preferred_name { get; set; }

        [JsonPropertyName("transaction_permissions")]
        public string transaction_permissions { get; set; }

        [JsonPropertyName("role_in_company")]
        public string role_in_company { get; set; }
    }

    public class CreateWalletResponse
    {
        [JsonPropertyName("status")]
        public Status status { get; set; }

        [JsonPropertyName("data")]
        public Data data { get; set; }
    }

    public class Status
    {
        [JsonPropertyName("error_code")]
        public string error_code { get; set; }

        [JsonPropertyName("status")]
        public string status { get; set; }

        [JsonPropertyName("message")]
        public string message { get; set; }

        [JsonPropertyName("response_code")]
        public string response_code { get; set; }

        [JsonPropertyName("operation_id")]
        public string operation_id { get; set; }
    }
}
