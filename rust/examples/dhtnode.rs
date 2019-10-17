/*
 *  Copyright (C) 2019 Savoir-faire Linux Inc.
 *  Author: Sébastien Blin <sebastien.blin@savoirfairelinux.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

extern crate opendht;
// TODO remove dead code warning
use std::{thread, time};
use libc::c_void;

use opendht::{InfoHash,DhtRunner,Value};

extern fn get_cb(v: *mut Value, ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let _handler: &mut Handler = unsafe { &mut *(ptr as *mut Handler) };
    unsafe {
        println!("Got data: {}", *v);
    }
}

extern fn value_cb(v: *mut Value, expired: bool, ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let _handler: &mut Handler = unsafe { &mut *(ptr as *mut Handler) };
    unsafe {
        println!("Got data: {} - expired: {}", *v, expired);
    }
}

extern fn done_cb(ok: bool, ptr: *mut c_void) {
    let _handler: &mut Handler = unsafe { &mut *(ptr as *mut Handler) };
    println!("In done - {}", ok);
}

struct Handler {
    _data: u8,
}


fn main() {
    println!("{}", InfoHash::random());
    println!("{}", InfoHash::new());
    // TODO inverted is_zero
    println!("{}", InfoHash::new().is_zero());
    println!("{}", InfoHash::get("alice"));
    println!("{}", InfoHash::get("alice").is_zero());

    let mut dht = DhtRunner::new();
    dht.run(1412);
    dht.bootstrap("bootstrap.jami.net", 4222);
    let ten_secs = time::Duration::from_secs(10);
    // TODO lambda instead
    let mut handler = Handler {
        _data: 8,
    };
    let ptr = &mut handler as *mut _ as *mut c_void;

    //println!("Start listening /foo");
    //let token = dht.listen(&InfoHash::get("foo"), value_cb, ptr);
    //thread::sleep(ten_secs);
    //println!("Stop listening /foo");
    //dht.cancel_listen(&InfoHash::get("foo"), token);
    //loop {
        println!("Get /alice");
        dht.get(&InfoHash::get("alice"), get_cb, done_cb, ptr);
        let v = Value::new("hi!");
        dht.put(&InfoHash::get("bob"), v, done_cb, ptr);
        //thread::sleep(ten_secs);
    //}
}