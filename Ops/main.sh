#!/bin/bash

log_message() {
    local level="$1"
    local message="$2"
    full_message="$(date '+%Y-%m-%d %H:%M:%S') - [$level] - $message"
    echo "$full_message" >> /var/log/network_ops.log
    echo "$full_message"
}

init_network() {
    log_message "INFO" "正在初始化网络配置..."
    cat <<EOF > /etc/network/interfaces
# 网络接口配置
auto lo
iface lo inet loopback

auto eth0
iface eth0 inet static
    address 172.22.146.150
    netmask 255.255.255.0
    gateway 172.22.146.1

auto eth1
iface eth1 inet dhcp
EOF
    systeemctl restart networking
    log_message "INFO" "网络配置已初始化。"
}

activate_network() {
    log_message "INFO" "正在激活 eth1 接口..."
    local eth1_address
    eth1_address=$(ip addr show eth1 | grep "inet " | awk '{print $2}' | cut -d/ -f1 || {
        log_message "ERROR" "eth1 接口或地址不存在，无法激活。"
        return 1
    })
    curl -X GET "http://192.168.202.2/?ip=$eth1_address" -o /dev/null || {
        log_message "ERROR" "无法通过 curl 命令激活 eth1 接口。"
        return 1
    }
    log_message "INFO" "eth1 接口已激活。"
}

configure_route() {
    log_message "INFO" "正在配置路由..."
    eth1_gateway=$(ip route show dev eth1 | grep default | awk '{print $3}' || {
        log_message "ERROR" "无法获取 eth1 的网关地址。"
        return 1
    })
    ip route add default via 172.22.146.1 dev eth0 || {
        log_message "ERROR" "路由配置失败。"
        return 1
    }
    ip route add 10.16.0.0/16 via "${eth1_gateway}" dev eth1 || {
        log_message "ERROR" "路由配置失败。"
        return 1
    }
    log_message "INFO" "路由配置完成。"
}

test_network() {
    log_message "INFO" "正在测试网络连接..."
    ping -I eth0 -c 3 223.5.5.5 &> /dev/null
    return $?
}

change_route() {
    log_message "INFO" "正在更改路由到 eth1..."
    eth1_gateway=$(ip route show dev eth1 | grep default | awk '{print $3}' || {
            log_message "ERROR" "无法获取 eth1 的网关地址。"
            return 1
    })
    ip route del default dev eth0 || {
        log_message "ERROR" "删除默认路由失败。"
        return 1
    }
    ip route add default via "$eth1_gateway" dev eth1 || {
        log_message "ERROR" "添加默认路由失败。"
        return 1
    }
    log_message "INFO" "路由更改完成。"
}

change_route_back() {
    log_message "INFO" "连接恢复正常，正在将路由更改回 eth0..."
    ip route del default dev eth1 || {
        log_message "ERROR" "删除默认路由失败。"
        return 1
    }
    ip route add default via 172.22.146.1 dev eth0 || {
        log_message "ERROR" "添加默认路由失败。"
        return 1
    }
    log_message "INFO" "路由更改回 eth0 完成。"
}

monitor_network() {
    log_message "INFO" "开始监控网络连接..."
    while true; do
        if ! test_network; then
            log_message "ERROR" "网络连接失败，正在重新配置..."
            change_route
            local fail=true
        else
            log_message "INFO" "网络连接正常。"
            if [ "$fail" ]; then
                change_route_back
                fail=false
            fi
        fi
        sleep 60
    done
}

main() {
    init_network
    activate_network
    configure_route
    monitor_network
}