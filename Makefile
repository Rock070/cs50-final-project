# -v $(PWD)/postgres-data:/var/lib/postgresql/data \

test-database-init:
	docker run --name shor-testing-postgres \
		-e POSTGRES_PASSWORD=postgres \
		-e POSTGRES_DB=shor_test \
		-p 8000:5432 \
		-d postgres
	@echo "等待 PostgreSQL 啟動..."
	@until docker exec shor-testing-postgres pg_isready -U postgres; do \
		echo "PostgreSQL 未就緒 - 等待..."; \
		sleep 1; \
	done
	@echo "PostgreSQL 已就緒!"
	sea-orm-cli migrate fresh -u postgres://postgres:postgres@localhost:8000/shor_test
	docker exec shor-testing-postgres psql -U postgres -d shor_test -c "\dt"



