
import logging
import sys
import time

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.INFO)


logging.info("hello to stdout BEFORE init-sleep")
time.sleep(2.0)
logging.info("hello to stdout AFTER init-sleep")

# logging.info("waiting for non-zero-length read on stdin...")
# while True:
#     tmp = sys.stdin.read()
#     if len(tmp) > 0:
#         break;
# logging.info("hello to stdout AFTER non-zero-length read on stdin")

logging.info("sleeping for 2s")
time.sleep(2.0)
# logging.info("hello to stdout AFTER post-read-sleep")
logging.info("bye-bye")

sys.stdout.flush()
sys.exit(0)

