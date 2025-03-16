import sys
from datetime import datetime, timedelta
import pytz

data = open(sys.argv[1]).read().strip()
lines = data.split('\n')

ans = 0
for i, line in enumerate(lines):
    parts = line.split()
    time, add, sub = parts[0], int(parts[1]), int(parts[2])

    not_utc_dt = datetime.strptime(time[:-6], "%Y-%m-%dT%H:%M:%S.%f")
    offset_hours = int(time[-6:-3])  # Extracts "-04"
    timezones = ["America/Halifax", "America/Santiago"]
    for timezone in timezones:
        tz = pytz.timezone(timezone)
        localized_dt = tz.localize(not_utc_dt, is_dst=None)  # Let pytz handle DST automatically
        if localized_dt.utcoffset().total_seconds() / 3600 == offset_hours:
            utc_dt = localized_dt.astimezone(pytz.utc)
            new_utc_dt = utc_dt + timedelta(minutes=-sub+add)
            new_localized_dt = new_utc_dt.astimezone(tz)
            ans += new_localized_dt.hour*i
            break
print(ans)
