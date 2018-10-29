(ns app.core
    (:require-macros
      [cljs.core.async.macros :as asyncm :refer (go go-loop)])
    (:require [reagent.core :as reagent :refer [atom]]
              [clojure.string :as str]
              [ajax.core :as ajx]
              [cljs.core.async :as async :refer (<! >! put! chan close!)]
              [wscljs.client :as ws]
              [wscljs.format :as fmt]))

(enable-console-print!)

(println "Console ready")

;; define your app data so that it doesn't get over-written on reload

(def wsurl "ws://localhost:8080/ws/")

(defonce app-state (atom {:text "Trivia"}))
(defonce ws-socket (atom nil))

;; TODO: factor out message as param
(defn send-msg [] (ws/send @ws-socket {"team_token" "team1"} fmt/json))

(def handlers {:on-message (fn [e] (prn (.-data e)))
               :on-open    #(prn "Opening a new connection")
               :on-close   #(prn "Closing a connection")})

;(defn start-connection []
;  (go
;    (let [socket (ws/create wsurl {:on-message identity})]
;      (is (= :connecting (ws/status socket)))
;      (<! (timeout 1000))
;      (is (= :open (ws/status socket)))
;      (done))))

; (def ws-socket (ws/create wsurl handlers))

(defn app []
  [:div.center
    [:h1 (:text @app-state)]
    [:h3.question-marker "Q: " [:span.question "What is...?"]]
    [:input {:type "button" :value "Click me!"
            :on-click #(send-msg)}]])

(reset! ws-socket (ws/create wsurl handlers))
(reagent/render-component [app]
                          (. js/document (getElementById "app")))

(defn on-js-reload []
  (do
    (println "reloaded...")
    ;; TODO: close previous websocket (or just open it once...)
    )
)

