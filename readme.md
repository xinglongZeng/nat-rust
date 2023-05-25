NAT is implemented with rust





client-A want connect client-B , use next step:

    1. create connection client-A/B to server , and keep heartbeat;; (创建客服端A（客户端B）和服务的链接，并且保持心跳)
    
    2. get ip address of another client from server; (客户端从服务端获取到另一个客户端的ip地址)
    
    3.  both client send "state of ready" to server , server send "command of nat" to  both client ; (服务端给客户端A、B发送打洞指令) 
    
