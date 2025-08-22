using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Agrochain.Sandbox.Dto
{
    public class CountryDetails
    {
        [JsonPropertyName("id")]
        public int Id { get; set; }

        [JsonPropertyName("name")]
        public string Name { get; set; }

        [JsonPropertyName("iso_alpha2")]
        public string IsoAlpha2 { get; set; }

        [JsonPropertyName("iso_alpha3")]
        public string IsoAlpha3 { get; set; }

        [JsonPropertyName("currency_code")]
        public string CurrencyCode { get; set; }

        [JsonPropertyName("currency_name")]
        public string CurrencyName { get; set; }

        [JsonPropertyName("currency_sign")]
        public string CurrencySign { get; set; }

        [JsonPropertyName("phone_code")]
        public string PhoneCode { get; set; }
    }
}
