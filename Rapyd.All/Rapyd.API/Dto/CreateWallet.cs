using System.Text.Json.Serialization;

namespace Agrochain.API.Dto
{
    public class Address
    {
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

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("canton")]
        public string canton { get; set; }

        [JsonPropertyName("district")]
        public string district { get; set; }
    }

    public class Contact
    {
        [JsonPropertyName("phone_number")]
        public string phone_number { get; set; }

        [JsonPropertyName("email")]
        public string email { get; set; }

        [JsonPropertyName("first_name")]
        public string first_name { get; set; }

        [JsonPropertyName("last_name")]
        public string last_name { get; set; }

        [JsonPropertyName("mothers_name")]
        public string mothers_name { get; set; }

        [JsonPropertyName("contact_type")]
        public string contact_type { get; set; }

        [JsonPropertyName("address")]
        public Address address { get; set; }

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

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }
    }

    public class CreateWallet
    {
        [JsonPropertyName("first_name")]
        public string first_name { get; set; }

        [JsonPropertyName("last_name")]
        public string last_name { get; set; }

        [JsonPropertyName("ewallet_reference_id")]
        public string ewallet_reference_id { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("type")]
        public string type { get; set; }

        [JsonPropertyName("contact")]
        public Contact contact { get; set; }
    }
}
