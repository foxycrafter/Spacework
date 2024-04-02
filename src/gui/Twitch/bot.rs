use twitchchat::{Runner, UserConfig};

  pub fn start_bot() {
      // Configura la información de autenticación del bot de Twitch
      let user_config = UserConfig::builder()
          .name("nombre_de_usuario_del_bot")
          .token("token_de_autenticacion_del_bot")
          .build()
          .unwrap();

      // Crea un nuevo Runner para el bot de Twitch
      let runner = Runner::new(user_config);

      // Inicia el bot de Twitch
      runner.run(|msg| {
          // Lógica para manejar los mensajes recibidos en el chat de Twitch
          println!("Mensaje recibido: {}", msg);
      }).unwrap();
  }