using System.Text.Json.Serialization;

namespace Agrochain.API.Dto.Response
{
    public class Ewallet
    {
        [JsonPropertyName("ewallet_id")]
        public string ewallet_id { get; set; }

        [JsonPropertyName("amount")]
        public int amount { get; set; }

        [JsonPropertyName("percent")]
        public int percent { get; set; }

        [JsonPropertyName("refunded_amount")]
        public int refunded_amount { get; set; }
    }

    public class Instructions
    {
    }

    public class PaymentMethodData
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("type")]
        public string type { get; set; }

        [JsonPropertyName("category")]
        public string category { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("image")]
        public string image { get; set; }

        [JsonPropertyName("webhook_url")]
        public string webhook_url { get; set; }

        [JsonPropertyName("supporting_documentation")]
        public string supporting_documentation { get; set; }

        [JsonPropertyName("next_action")]
        public string next_action { get; set; }

        [JsonPropertyName("name")]
        public string name { get; set; }

        [JsonPropertyName("last4")]
        public string last4 { get; set; }

        [JsonPropertyName("acs_check")]
        public string acs_check { get; set; }

        [JsonPropertyName("cvv_check")]
        public string cvv_check { get; set; }

        [JsonPropertyName("bin_details")]
        public BinDetails bin_details { get; set; }

        [JsonPropertyName("expiration_year")]
        public string expiration_year { get; set; }

        [JsonPropertyName("expiration_month")]
        public string expiration_month { get; set; }

        [JsonPropertyName("fingerprint_token")]
        public string fingerprint_token { get; set; }
    }

    public class PaymentMethodOptions
    {
    }

    public class RemitterInformation
    {
    }

    public class CardPaymentResponse
    {
        [JsonPropertyName("id")]
        public string id { get; set; }

        [JsonPropertyName("amount")]
        public int amount { get; set; }

        [JsonPropertyName("original_amount")]
        public int original_amount { get; set; }

        [JsonPropertyName("is_partial")]
        public bool is_partial { get; set; }

        [JsonPropertyName("currency_code")]
        public string currency_code { get; set; }

        [JsonPropertyName("country_code")]
        public string country_code { get; set; }

        [JsonPropertyName("status")]
        public string status { get; set; }

        [JsonPropertyName("description")]
        public string description { get; set; }

        [JsonPropertyName("merchant_reference_id")]
        public string merchant_reference_id { get; set; }

        [JsonPropertyName("customer_token")]
        public string customer_token { get; set; }

        [JsonPropertyName("payment_method")]
        public string payment_method { get; set; }

        [JsonPropertyName("payment_method_data")]
        public PaymentMethodData payment_method_data { get; set; }

        [JsonPropertyName("auth_code")]
        public object auth_code { get; set; }

        [JsonPropertyName("expiration")]
        public int expiration { get; set; }

        [JsonPropertyName("captured")]
        public bool captured { get; set; }

        [JsonPropertyName("refunded")]
        public bool refunded { get; set; }

        [JsonPropertyName("refunded_amount")]
        public int refunded_amount { get; set; }

        [JsonPropertyName("receipt_email")]
        public string receipt_email { get; set; }

        [JsonPropertyName("redirect_url")]
        public string redirect_url { get; set; }

        [JsonPropertyName("complete_payment_url")]
        public string complete_payment_url { get; set; }

        [JsonPropertyName("error_payment_url")]
        public string error_payment_url { get; set; }

        [JsonPropertyName("receipt_number")]
        public string receipt_number { get; set; }

        [JsonPropertyName("flow_type")]
        public string flow_type { get; set; }

        [JsonPropertyName("address")]
        public object address { get; set; }

        [JsonPropertyName("statement_descriptor")]
        public string statement_descriptor { get; set; }

        [JsonPropertyName("transaction_id")]
        public string transaction_id { get; set; }

        [JsonPropertyName("created_at")]
        public int created_at { get; set; }

        [JsonPropertyName("metadata")]
        public Metadata metadata { get; set; }

        [JsonPropertyName("failure_code")]
        public string failure_code { get; set; }

        [JsonPropertyName("failure_message")]
        public string failure_message { get; set; }

        [JsonPropertyName("paid")]
        public bool paid { get; set; }

        [JsonPropertyName("paid_at")]
        public int paid_at { get; set; }

        [JsonPropertyName("dispute")]
        public object dispute { get; set; }

        [JsonPropertyName("refunds")]
        public object refunds { get; set; }

        [JsonPropertyName("order")]
        public object order { get; set; }

        [JsonPropertyName("outcome")]
        public object outcome { get; set; }

        [JsonPropertyName("visual_codes")]
        public VisualCodes visual_codes { get; set; }

        [JsonPropertyName("textual_codes")]
        public TextualCodes textual_codes { get; set; }

        [JsonPropertyName("instructions")]
        public Instructions instructions { get; set; }

        [JsonPropertyName("ewallet_id")]
        public string ewallet_id { get; set; }

        [JsonPropertyName("ewallets")]
        public List<Ewallet> ewallets { get; set; }

        [JsonPropertyName("payment_method_options")]
        public PaymentMethodOptions payment_method_options { get; set; }

        [JsonPropertyName("payment_method_type")]
        public string payment_method_type { get; set; }

        [JsonPropertyName("payment_method_type_category")]
        public string payment_method_type_category { get; set; }

        [JsonPropertyName("fx_rate")]
        public int fx_rate { get; set; }

        [JsonPropertyName("merchant_requested_currency")]
        public object merchant_requested_currency { get; set; }

        [JsonPropertyName("merchant_requested_amount")]
        public object merchant_requested_amount { get; set; }

        [JsonPropertyName("fixed_side")]
        public string fixed_side { get; set; }

        [JsonPropertyName("payment_fees")]
        public object payment_fees { get; set; }

        [JsonPropertyName("invoice")]
        public string invoice { get; set; }

        [JsonPropertyName("escrow")]
        public object escrow { get; set; }

        [JsonPropertyName("group_payment")]
        public string group_payment { get; set; }

        [JsonPropertyName("cancel_reason")]
        public object cancel_reason { get; set; }

        [JsonPropertyName("initiation_type")]
        public string initiation_type { get; set; }

        [JsonPropertyName("mid")]
        public string mid { get; set; }

        [JsonPropertyName("next_action")]
        public string next_action { get; set; }

        [JsonPropertyName("error_code")]
        public string error_code { get; set; }

        [JsonPropertyName("remitter_information")]
        public RemitterInformation remitter_information { get; set; }
    }

    public class TextualCodes
    {
    }

    public class VisualCodes
    {
    }


}
