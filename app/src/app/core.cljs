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

(def wsurl "ws://localhost:3200/ws/")
(defonce app-state (atom {:text "Trivia"}))

(defn app []
  [:div.center
    [:h1 (:text @app-state)]
    [:h3.question-marker "Q: " [:span.question "What is...?"]]])

(reagent/render-component [app]
                          (. js/document (getElementById "app")))

;(defn start-connection []
;  (go
;    (let [socket (ws/create wsurl {:on-message identity})]
;      (is (= :connecting (ws/status socket)))
;      (<! (timeout 1000))
;      (is (= :open (ws/status socket)))
;      (done))))

;(defonce router_ (atom nil))
;(defn stop-router! [] (when-let [stop-f @router_] (stop-f)))
;(defn start-router! []
;      (stop-router!)
;      (reset! router_ 
;        (start-connection)))

(defn start!
      []
;      (start-router!)
      (reagent/render-component [app]
        (.getElementById js/document "app")))

;(defn ^:export run []
;  (reagent/render [app]
;            (js/document.getElementById "app")))        

(defn on-js-reload []
  ;; optionally touch your app-state to force rerendering depending on
  ;; your application
  ;; (swap! app-state update-in [:__figwheel_counter] inc)
)
