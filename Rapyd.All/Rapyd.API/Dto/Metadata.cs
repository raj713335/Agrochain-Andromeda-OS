using System.Text.Json.Serialization;

namespace Agrochain.API.Dto
{
    public class Metadata
    {
        [JsonPropertyName("merchant_defined")]
        public bool merchant_defined { get; set; }
    }
}
