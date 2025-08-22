using System;
using System.Collections.Generic;

namespace AgrochainIOT.AzureDB
{
    public partial class SensorDatum
    {
        public int? Co2InPpm { get; set; }
        public DateTime? EventEnqueuedUtcTime { get; set; }

        public DateTime LocalDateTime
        {
            get
            {
                return UTCTimeStampToDateTime(EventEnqueuedUtcTime);
            }
        }

        private DateTime UTCTimeStampToDateTime(DateTime? EventEnqueuedUtcTime)
        {
            TimeZoneInfo istZone = TimeZoneInfo.FindSystemTimeZoneById("India Standard Time");
            return TimeZoneInfo.ConvertTimeFromUtc(EventEnqueuedUtcTime ?? default, istZone);
        }
    }
}
