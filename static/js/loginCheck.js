// Проверяем наличие токена в localStorage
const authToken = localStorage.getItem('authToken');

if (!authToken) {
 // Если токена нет, перенаправляем на страницу входа
 window.location.href = '/login';
} else {
 // Отправляем запрос к API для проверки токена
 fetch('/api/check_admin_token', {
   method: 'POST',
   headers: {
     'Content-Type': 'application/json'
   },
   body: JSON.stringify({ 
     token: authToken
   })
 })
 .then(response => {
   if (!response.ok) {
     throw new Error('Ответ сети был неудовлетворительным:');
   }
   return response.json();
   window.location.href = '/login';
 })
 .then(data => {
   if (!data.token_valid) {
     // Если токен не валиден, перенаправляем на страницу входа
     window.location.href = '/login';
   }
 })
 .catch(error => {
   console.error('Возникла проблема с операцией выборки:', error);
   // В случае ошибки также перенаправляем на страницу входа
   window.location.href = '/login';
 });
}
