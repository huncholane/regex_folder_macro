# This will create a struct called Event with the fields start, end, ampm, and activity

# Get the start
(?<start>\d{1,2}:\d{1,2})
.
# Get the end time
(?<end>\d{1,2}:\d{1,2})
\s{0,2}
# Get the AM or PM after the times
(?<ampm>[AMPM]{2})
.
# Get the activity string
(?<activity>.+)
