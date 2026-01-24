from django.urls import path
from . import views

urlpatterns = [
    path('me', views.get_current_user, name='current_user'),
]
