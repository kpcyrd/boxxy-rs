#!/usr/bin/env python3
import subprocess
import logging
import base64
logger = logging.getLogger()
logger.setLevel(logging.INFO)

def run(event, context):
    #logger.info('got event {}'.format(event))
    p = subprocess.run(['./boxxy'],
        input=event['stdin'].encode('utf-8'),
        stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout = base64.b64encode(p.stdout).decode('utf-8')
    stderr = base64.b64encode(p.stderr).decode('utf-8')

    return {
        'stdout': stdout,
        'stderr': stderr,
    }

if __name__ == '__main__':
    print(run({
        'stdin': 'id'
    }, None))
