from fastapi import FastAPI

from personal_aid.health import router as health_router

app = FastAPI(title="Personal Aid API", version="0.1.0")
app.include_router(health_router)
