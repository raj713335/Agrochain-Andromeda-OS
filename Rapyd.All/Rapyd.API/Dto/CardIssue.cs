namespace Agrochain.API.Dto
{
    public class CardIssue
    {
        public string card_program { get; set; }
        public string? country { get; set; }
        public string ewallet_contact { get; set; }
        public int? expiration_month { get; set; }
        public int? expiration_year { get; set; }
    }
}
