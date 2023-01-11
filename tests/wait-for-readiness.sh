set -xe

wait_for_readiness() {
	local SERVICE="$1"
	local PORT="$2"
	local TRY_TIMEOUT=300
	local TRY_INTERVAL=1
	local REMAINING_TIME=$TRY_TIMEOUT
	while ! curl http://localhost:${PORT}/api/health-check -s --include | head -n1 | grep -q 200; do
		REMAINING_TIME=$((REMAINING_TIME - TRY_INTERVAL))
		if [ $REMAINING_TIME -lt 0 ]; then
			echo "Error: '${SERVICE}' did not start in expected duration."
			exit 1
		fi
		echo "Waiting for '${SERVICE}' to start... remaning ${REMAINING_TIME} seconds."
		sleep $TRY_INTERVAL
	done
	echo "The '${SERVICE}' is ready to be tested."
}

wait_for_readiness 'API Server' 3100
