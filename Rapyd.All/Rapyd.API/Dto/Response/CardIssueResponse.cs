using System.Text.Json.Serialization;

namespace Agrochain.API.Dto.Response
{
    public class CardDetails
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

        [JsonPropertyName("card_tracking_id")]
        public object card_tracking_id { get; set; }

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

    public class EwalletContact
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("first_name")]
        public string first_name { get; set; }

        [JsonPropertyName("last_name")]
        public string last_name { get; set; }

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

        [JsonPropertyName("phone_number")]
        public string phone_number { get; set; }

        [JsonPropertyName("email")]
        public string email { get; set; }

        [JsonPropertyName("identification_type")]
        public string identification_type { get; set; }

        [JsonPropertyName("identification_number")]
        public string identification_number { get; set; }

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

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("business_details")]
        public object business_details { get; set; }

        [JsonPropertyName("compliance_profile")]
        public int compliance_profile { get; set; }

        [JsonPropertyName("verification_status")]
        public string verification_status { get; set; }

        [JsonPropertyName("send_notifications")]
        public bool send_notifications { get; set; }

        [JsonPropertyName("mothers_name")]
        public string mothers_name { get; set; }
    }

    public class CardIssueResponse
    {
        [JsonPropertyName("status")]
        public Status status { get; set; }

        [JsonPropertyName("data")]
        public CardDetails data { get; set; }
    }
}
