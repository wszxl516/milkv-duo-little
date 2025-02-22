#!/bin/sh

case $1 in
	start)
		echo -n start > /sys/class/remoteproc/remoteproc0/state
		echo started
		;;
	stop)
		echo -n stop > /sys/class/remoteproc/remoteproc0/state
		echo stopped
		;;
	restart)
		$0 stop
		sleep 1
		$0 start
		;;
	status)
		cat /sys/class/remoteproc/remoteproc0/firmware
		cat /sys/class/remoteproc/remoteproc0/state
		;;
	*)
		echo "Usage: {start|stop|restart|status}"
		exit 1
		;;
esac

exit 0
