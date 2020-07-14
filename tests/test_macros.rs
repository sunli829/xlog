use xlog::*;

#[test]
fn test_macros() {
    simple_logger::init().unwrap();

    trace!("msg",);
    trace!("msg {}", 1,);
    trace!("msg {}", 1, a = 10, b = 20,);

    trace!(target: "abc", "msg",);

    trace!(target = "abc", "msg",);
    trace!(target = "abc", "msg {}", 1,);

    trace!(target = "abc", "msg", a = 10,);
    trace!(target = "abc", "msg", a = 10, b = 20,);

    trace!(target = "abc", "msg {}", 1, a = 10);
    trace!(target = "abc", "msg {}", 1, a = 10, b = 20);

    trace!("msg");
    trace!("msg {}", 1);
    trace!("msg {}", 1, a = 10, b = 20);

    trace!(target: "abc", "msg");

    trace!(target = "abc", "msg");
    trace!(target = "abc", "msg {}", 1);

    trace!(target = "abc", "msg", a = 10);
    trace!(target = "abc", "msg", a = 10, b = 20);

    trace!(target = "abc", "msg {}", 1, a = 10);
    trace!(target = "abc", "msg {}", 1, a = 10, b = 20);

    debug!("msg");
    debug!("msg {}", 1);
    debug!("msg {}", 1, a = 10, b = 20);

    debug!(target: "abc", "msg");

    debug!(target = "abc", "msg");
    debug!(target = "abc", "msg {}", 1);

    debug!(target = "abc", "msg", a = 10);
    debug!(target = "abc", "msg", a = 10, b = 20);

    debug!(target = "abc", "msg {}", 1, a = 10);
    debug!(target = "abc", "msg {}", 1, a = 10, b = 20);

    info!("msg");
    info!("msg {}", 1);
    info!("msg {}", 1, a = 10, b = 20);

    info!(target: "abc", "msg");

    info!(target = "abc", "msg");
    info!(target = "abc", "msg {}", 1);

    info!(target = "abc", "msg", a = 10);
    info!(target = "abc", "msg", a = 10, b = 20);

    info!(target = "abc", "msg {}", 1, a = 10);
    info!(target = "abc", "msg {}", 1, a = 10, b = 20);

    warn!("msg");
    warn!("msg {}", 1);
    warn!("msg {}", 1, a = 10, b = 20);

    warn!(target: "abc", "msg");

    warn!(target = "abc", "msg");
    warn!(target = "abc", "msg {}", 1);

    warn!(target = "abc", "msg", a = 10);
    warn!(target = "abc", "msg", a = 10, b = 20);

    warn!(target = "abc", "msg {}", 1, a = 10);
    warn!(target = "abc", "msg {}", 1, a = 10, b = 20);

    error!("msg");
    error!("msg {}", 1);
    error!("msg {}", 1, a = 10, b = 20);

    error!(target: "abc", "msg");

    error!(target = "abc", "msg");
    error!(target = "abc", "msg {}", 1);

    error!(target = "abc", "msg", a = 10);
    error!(target = "abc", "msg", a = 10, b = 20);

    error!(target = "abc", "msg {}", 1, a = 10);
    error!(target = "abc", "msg {}", 1, a = 10, b = 20);
}
