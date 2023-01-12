deploy:
	git config credential.helper store
	git pull
	docker-compose build
	docker-compose up -d --remove-orphans

restart:
	docker-compose restart
