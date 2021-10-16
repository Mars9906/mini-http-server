#  mini http server个人理解
  * 一开始，accept函数对应的future obj 阻塞在等待连接这个操作上，此时线程也没有其他的future执行。等到建立起了一个tcp连接之后，accept函数对应的future obj 变成ready状态，获取线程控制权，得到执行，返回一个tcp客户端和tcp服务端对应的tcp stream，然后另起一个future obj去对这个 tcp stream 进行处理，然后此时是有了两个future obj，然后accept函数对应的future obj 又是阻塞在等待连接这个操作上进入pending状态，失去线程控制权， 处理tcp stream 的future obj 此时是ready的，自然就获取了线程控制权得到执行。
  
  * 一个线程可以运行多个future obj，当一个future obj因为调用了同步io操作阻塞时，进入pending状态，失去线程控制权，此时另一个future obj获取线程控制权，得到执行，可能执行到设置的时间限制后，又会让出给进入ready状态的其他future obj。 不使用多线程但是却得到了多线程的效果。 

  * 使用浏览器作为客户端，在地址栏输入127.0.0.1 的话 , 相当于于 是使用get方法请求127.0.0.1这个 http 服务器的 / 资源(请求资源路径是服务器根目录)，然后服务器在接受到请求之后会路由处理，即根据请求资源返回相应的资源。

# 压力测试相关
  ```lsof -i:port```
  ```top -p pid ```
  ```webbench -c 并发数 -t 运行测试时间 URL```

# 参考资料
[rust 异步编程相关资料](https://www.cnblogs.com/dhcn/p/12950474.html)