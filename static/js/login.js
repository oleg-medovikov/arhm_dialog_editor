document.getElementById('loginForm').addEventListener('submit', async function(event) {
 event.preventDefault(); // Предотвращаем отправку формы

 try {
   const response = await fetch('/api/admin_login', {
     method: 'POST',
     headers: {
       'Content-Type': 'application/json',
     },
     body: JSON.stringify({
       username: event.target.username.value,
       password: event.target.password.value,
     }),
   });

   if (!response.ok) {
     throw new Error(`Ошибка сервера: ${response.status} ${response.statusText}`);
   }

   const data = await response.json();

   // Сохраняем токен в localStorage
   localStorage.setItem('authToken', data.token);

   // Перенаправляем пользователя на главную страницу
   window.location.href = '/';
 } catch (error) {
   console.error('Возникла проблема с операцией выборки:', error);

   // Показываем пользователю сообщение об ошибке
   alert('Ошибка входа. Пожалуйста, проверьте свои учетные данные.');
 }
});
